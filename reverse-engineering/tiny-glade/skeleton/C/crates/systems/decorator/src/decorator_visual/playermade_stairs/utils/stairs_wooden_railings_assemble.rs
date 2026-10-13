// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 310c0e6be5a7b85afd991234bcb5f3dbd08e2fa066268e10cbab0e3274d01dad
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_wooden_railings_assemble.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 432, 466, 1545, 1557, 1588, 1600
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":272,"rva":9120880,"va":5377830000}
// 编译实例 RVAs: 0x8b2c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2152748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8d426dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377831431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-2152955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e80526dfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377831654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9109225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8170175ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377831412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8593129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e817e17cff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377833732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089e60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8559689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8b7637dff","size":5,"soft_fail":false,"target_va":5369273952,"va":5377833636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4256770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8fe0bbfff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377831533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141498980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12474044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8bc56be00","size":5,"soft_fail":false,"target_va":5390305664,"va":5377831615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a7280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12534325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e83542bf00","size":5,"soft_fail":false,"target_va":5390365312,"va":5377830982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39225551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8cf885602","size":5,"soft_fail":false,"target_va":5417057056,"va":5377831500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38548898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8a2354c02","size":5,"soft_fail":false,"target_va":5416381680,"va":5377832777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38788276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8b4dc4f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377832513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38788663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e837de4f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377832932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38788270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8aedc4f02","size":5,"soft_fail":false,"target_va":5416621188,"va":5377832913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t38788745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e889de4f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377833012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<array$<enum2$<core::option::Option<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global> > >,2> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8b1660000","size":5,"soft_fail":false,"target_va":5377859808,"va":5377833514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wooden_stair_support::inset_wooden_stair_terminal_station_points
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2351812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377830000,"raw_hex":"e8c4e22300","size":5,"soft_fail":false,"target_va":5380183664,"va":5377831847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":5344,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":6268,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":2212,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":1836,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":3844,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":1192,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":1092,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":1540,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":1752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":1696,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":3996,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":3516,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":466,"record_offset":2836,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":4228,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":3968,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":5260,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":2056,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":1944,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::floor_offsets
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":466,"record_offset":556,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27386,"source_line":20,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::set_side_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_railing_shared.rs
//     evidence={"module":466,"record_offset":2168,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27490,"source_line":27,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::side_positions
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_railing_shared.rs
//     evidence={"module":466,"record_offset":3816,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27489,"source_line":23,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":2612,"parent_record_offset":2432,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3084,"parent_record_offset":2992,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":6372,"parent_record_offset":6340,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2372,"parent_record_offset":2344,"ancestor_record_offset":272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2468,"parent_record_offset":2432,"ancestor_record_offset":272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":2252,"parent_record_offset":2212,"ancestor_record_offset":272,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":2292,"parent_record_offset":2252,"ancestor_record_offset":272,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":6340,"parent_record_offset":6296,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":6296,"parent_record_offset":6268,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":2796,"parent_record_offset":2292,"ancestor_record_offset":272,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":2344,"parent_record_offset":2292,"ancestor_record_offset":272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":466,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":466,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":272,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":272,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":660,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":700,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":1600,"parent_record_offset":1568,"ancestor_record_offset":272,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":466,"record_offset":2572,"parent_record_offset":2516,"ancestor_record_offset":272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":272,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":1632,"parent_record_offset":1568,"ancestor_record_offset":272,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":272,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":466,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":466,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":2544,"parent_record_offset":2516,"ancestor_record_offset":272,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":272,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":2100,"parent_record_offset":2056,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":1988,"parent_record_offset":1944,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":466,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":272,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":466,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":272,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":466,"record_offset":2740,"parent_record_offset":2292,"ancestor_record_offset":272,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":6460,"parent_record_offset":6296,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":272,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":3872,"parent_record_offset":3844,"ancestor_record_offset":272,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":1900,"parent_record_offset":1868,"ancestor_record_offset":272,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":272,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":1124,"parent_record_offset":1092,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":466,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":272,"inlinee":27388,"source_line":21,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":466,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":272,"inlinee":27390,"source_line":23,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":272,"inlinee":27529,"source_line":86,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":272,"inlinee":27530,"source_line":128,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B89
// 首条位置: {"module":466,"inlinee":27529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13B4B
// 首条位置: {"module":1600,"inlinee":80715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":33812,"parent_record_offset":33548,"ancestor_record_offset":33548,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1600,"record_offset":33756,"parent_record_offset":33548,"ancestor_record_offset":33548,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":33612,"parent_record_offset":33548,"ancestor_record_offset":33548,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":33668,"parent_record_offset":33548,"ancestor_record_offset":33548,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":33696,"parent_record_offset":33668,"ancestor_record_offset":33548,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":33548,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":33780,"parent_record_offset":33756,"ancestor_record_offset":33548,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B8A
// 首条位置: {"module":466,"inlinee":27530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2868,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2868,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2868,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":3084,"parent_record_offset":2992,"ancestor_record_offset":2868,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":2868,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2868,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":2868,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":2868,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":39016,"rva":9139504,"va":5377848624}
// 编译实例 RVAs: 0x8b7530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=34
//     evidence={"assembly":"\tcallq\t-2172094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e842dbdeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377850793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089e60
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8577059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8dd1f7dff","size":5,"soft_fail":false,"target_va":5369273952,"va":5377851006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4276807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8b9bdbeff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377851570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414991f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12454074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8ba08be00","size":5,"soft_fail":false,"target_va":5390307824,"va":5377853745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499e70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12458710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8d61abe00","size":5,"soft_fail":false,"target_va":5390311024,"va":5377852309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a8f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12512906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e88aeebe00","size":5,"soft_fail":false,"target_va":5390372704,"va":5377859793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a92e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12515254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8b6f7be00","size":5,"soft_fail":false,"target_va":5390373600,"va":5377858341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38498084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8246f4b02","size":5,"soft_fail":false,"target_va":5416352064,"va":5377853975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38502897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8f1814b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5377850138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39204429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e84d365602","size":5,"soft_fail":false,"target_va":5417057664,"va":5377853230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38532181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e855f44b02","size":5,"soft_fail":false,"target_va":5416381680,"va":5377849494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38766001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8b1854f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377855594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38766980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e884894f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377855223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t38766202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e87a864f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377855555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wooden_stair_support::resample_wooden_support_station_points
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2331527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e887932300","size":5,"soft_fail":false,"target_va":5380183232,"va":5377851700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e862c8ffff","size":5,"soft_fail":false,"target_va":5377840496,"va":5377854729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e872b3ffff","size":5,"soft_fail":false,"target_va":5377835232,"va":5377854825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5428173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848624,"raw_hex":"e8cdd35200","size":5,"soft_fail":false,"target_va":5383281568,"va":5377853390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-2172451","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377848624,"raw_hex":"e9ddd9deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377851150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":39316,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":39344,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":41072,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":39372,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":40216,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":466,"record_offset":39428,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::array::from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":47872,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":26753,"source_line":109,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":50148,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":53952,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":48012,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":466,"record_offset":51972,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":42024,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":42168,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":54100,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":39604,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":52564,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":43024,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":39576,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":47984,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":50092,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":50176,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":466,"record_offset":51244,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":51416,"parent_record_offset":39016,"ancestor_record_offset":39016,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":43236,"parent_record_offset":43212,"ancestor_record_offset":39016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":43260,"parent_record_offset":43236,"ancestor_record_offset":39016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":43284,"parent_record_offset":43260,"ancestor_record_offset":39016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":43212,"parent_record_offset":43124,"ancestor_record_offset":39016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40944,"parent_record_offset":40920,"ancestor_record_offset":39016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40424,"parent_record_offset":40396,"ancestor_record_offset":39016,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40508,"parent_record_offset":40484,"ancestor_record_offset":39016,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":43152,"parent_record_offset":43124,"ancestor_record_offset":39016,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":43124,"parent_record_offset":43100,"ancestor_record_offset":39016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40484,"parent_record_offset":40452,"ancestor_record_offset":39016,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40992,"parent_record_offset":40968,"ancestor_record_offset":39016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40968,"parent_record_offset":40944,"ancestor_record_offset":39016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":40452,"parent_record_offset":40424,"ancestor_record_offset":39016,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":43100,"parent_record_offset":43076,"ancestor_record_offset":39016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":40920,"parent_record_offset":40288,"ancestor_record_offset":39016,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":40288,"parent_record_offset":40252,"ancestor_record_offset":39016,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":41104,"parent_record_offset":41072,"ancestor_record_offset":39016,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":40396,"parent_record_offset":40288,"ancestor_record_offset":39016,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":39788,"parent_record_offset":39652,"ancestor_record_offset":39016,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":40756,"parent_record_offset":40728,"ancestor_record_offset":39016,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":44236,"parent_record_offset":44208,"ancestor_record_offset":39016,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":466,"record_offset":44896,"parent_record_offset":44856,"ancestor_record_offset":39016,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":466,"record_offset":44856,"parent_record_offset":44828,"ancestor_record_offset":39016,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":466,"record_offset":45148,"parent_record_offset":45124,"ancestor_record_offset":39016,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":466,"record_offset":44968,"parent_record_offset":44944,"ancestor_record_offset":39016,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":466,"record_offset":40836,"parent_record_offset":40756,"ancestor_record_offset":39016,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":466,"record_offset":41032,"parent_record_offset":40288,"ancestor_record_offset":39016,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":466,"record_offset":40252,"parent_record_offset":40216,"ancestor_record_offset":39016,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":466,"record_offset":47896,"parent_record_offset":47872,"ancestor_record_offset":39016,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":50488,"parent_record_offset":50464,"ancestor_record_offset":39016,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":50512,"parent_record_offset":50488,"ancestor_record_offset":39016,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":42076,"parent_record_offset":42048,"ancestor_record_offset":39016,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":50028,"parent_record_offset":50000,"ancestor_record_offset":39016,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":50116,"parent_record_offset":50092,"ancestor_record_offset":39016,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":49252,"parent_record_offset":49216,"ancestor_record_offset":39016,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":40700,"parent_record_offset":40656,"ancestor_record_offset":39016,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":40592,"parent_record_offset":40560,"ancestor_record_offset":39016,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":40340,"parent_record_offset":40288,"ancestor_record_offset":39016,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::flatten_one::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":466,"record_offset":48172,"parent_record_offset":48140,"ancestor_record_offset":39016,"inlinee":27560,"source_line":806,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$11::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":466,"record_offset":48044,"parent_record_offset":48012,"ancestor_record_offset":39016,"inlinee":27557,"source_line":245,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$46::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":466,"record_offset":48076,"parent_record_offset":48044,"ancestor_record_offset":39016,"inlinee":27558,"source_line":881,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":466,"record_offset":48108,"parent_record_offset":48076,"ancestor_record_offset":39016,"inlinee":10924,"source_line":93,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":48212,"parent_record_offset":48172,"ancestor_record_offset":39016,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":40624,"parent_record_offset":40592,"ancestor_record_offset":39016,"inlinee":15216,"source_line":122,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":48832,"parent_record_offset":48284,"ancestor_record_offset":39016,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":40364,"parent_record_offset":40340,"ancestor_record_offset":39016,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48924,"parent_record_offset":48832,"ancestor_record_offset":39016,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48884,"parent_record_offset":48832,"ancestor_record_offset":39016,"inlinee":27534,"source_line":46,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":466,"record_offset":51996,"parent_record_offset":51972,"ancestor_record_offset":39016,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":42104,"parent_record_offset":42048,"ancestor_record_offset":39016,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":39016,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":466,"record_offset":44944,"parent_record_offset":44920,"ancestor_record_offset":39016,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":466,"record_offset":44920,"parent_record_offset":44896,"ancestor_record_offset":39016,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40044,"parent_record_offset":39652,"ancestor_record_offset":39016,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40100,"parent_record_offset":40072,"ancestor_record_offset":39016,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40560,"parent_record_offset":40288,"ancestor_record_offset":39016,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40728,"parent_record_offset":40700,"ancestor_record_offset":39016,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40072,"parent_record_offset":40044,"ancestor_record_offset":39016,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":42128,"parent_record_offset":42104,"ancestor_record_offset":39016,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":43180,"parent_record_offset":43152,"ancestor_record_offset":39016,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":41940,"parent_record_offset":41916,"ancestor_record_offset":39016,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":39016,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48728,"parent_record_offset":48700,"ancestor_record_offset":39016,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":39948,"parent_record_offset":39908,"ancestor_record_offset":39016,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":466,"record_offset":50000,"parent_record_offset":48212,"ancestor_record_offset":39016,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":466,"record_offset":50464,"parent_record_offset":50440,"ancestor_record_offset":39016,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":466,"record_offset":50440,"parent_record_offset":50404,"ancestor_record_offset":39016,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":40160,"parent_record_offset":40132,"ancestor_record_offset":39016,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":39752,"parent_record_offset":39652,"ancestor_record_offset":39016,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":466,"record_offset":42740,"parent_record_offset":42716,"ancestor_record_offset":39016,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":43052,"parent_record_offset":43024,"ancestor_record_offset":39016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":39396,"parent_record_offset":39372,"ancestor_record_offset":39016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":42716,"parent_record_offset":42692,"ancestor_record_offset":39016,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":39488,"parent_record_offset":39428,"ancestor_record_offset":39016,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":42692,"parent_record_offset":42632,"ancestor_record_offset":39016,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":39516,"parent_record_offset":39428,"ancestor_record_offset":39016,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":39460,"parent_record_offset":39428,"ancestor_record_offset":39016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":39016,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":40788,"parent_record_offset":40756,"ancestor_record_offset":39016,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":39724,"parent_record_offset":39652,"ancestor_record_offset":39016,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":39816,"parent_record_offset":39788,"ancestor_record_offset":39016,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":39844,"parent_record_offset":39816,"ancestor_record_offset":39016,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":40656,"parent_record_offset":40624,"ancestor_record_offset":39016,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":53980,"parent_record_offset":53952,"ancestor_record_offset":39016,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":52028,"parent_record_offset":51996,"ancestor_record_offset":39016,"inlinee":17069,"source_line":418,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":48968,"parent_record_offset":48284,"ancestor_record_offset":39016,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49136,"parent_record_offset":49012,"ancestor_record_offset":39016,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49012,"parent_record_offset":48968,"ancestor_record_offset":39016,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39884,"parent_record_offset":39652,"ancestor_record_offset":39016,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39976,"parent_record_offset":39908,"ancestor_record_offset":39016,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48700,"parent_record_offset":48372,"ancestor_record_offset":39016,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48764,"parent_record_offset":48372,"ancestor_record_offset":39016,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39908,"parent_record_offset":39884,"ancestor_record_offset":39016,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":49216,"parent_record_offset":49136,"ancestor_record_offset":39016,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":39652,"parent_record_offset":39604,"ancestor_record_offset":39016,"inlinee":27554,"source_line":188,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":40132,"parent_record_offset":40100,"ancestor_record_offset":39016,"inlinee":27556,"source_line":196,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":48248,"parent_record_offset":48212,"ancestor_record_offset":39016,"inlinee":27561,"source_line":244,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::polyline_length_between_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":48284,"parent_record_offset":48248,"ancestor_record_offset":39016,"inlinee":27562,"source_line":301,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":466,"record_offset":50404,"parent_record_offset":50176,"ancestor_record_offset":39016,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":48372,"parent_record_offset":48284,"ancestor_record_offset":39016,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BA2
// 首条位置: {"module":466,"inlinee":27554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":39788,"parent_record_offset":39652,"ancestor_record_offset":39652,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40044,"parent_record_offset":39652,"ancestor_record_offset":39652,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":39752,"parent_record_offset":39652,"ancestor_record_offset":39652,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":39724,"parent_record_offset":39652,"ancestor_record_offset":39652,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39884,"parent_record_offset":39652,"ancestor_record_offset":39652,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40100,"parent_record_offset":40072,"ancestor_record_offset":39652,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":40072,"parent_record_offset":40044,"ancestor_record_offset":39652,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":39948,"parent_record_offset":39908,"ancestor_record_offset":39652,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":40160,"parent_record_offset":40132,"ancestor_record_offset":39652,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":39816,"parent_record_offset":39788,"ancestor_record_offset":39652,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":39844,"parent_record_offset":39816,"ancestor_record_offset":39652,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39976,"parent_record_offset":39908,"ancestor_record_offset":39652,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":39908,"parent_record_offset":39884,"ancestor_record_offset":39652,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":40132,"parent_record_offset":40100,"ancestor_record_offset":39652,"inlinee":27556,"source_line":196,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BA4
// 首条位置: {"module":466,"inlinee":27556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":40160,"parent_record_offset":40132,"ancestor_record_offset":40132,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BA9
// 首条位置: {"module":466,"inlinee":27561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::polyline_length_between_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":48284,"parent_record_offset":48248,"ancestor_record_offset":48248,"inlinee":27562,"source_line":301,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":49252,"parent_record_offset":49216,"ancestor_record_offset":48248,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":48832,"parent_record_offset":48284,"ancestor_record_offset":48248,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48924,"parent_record_offset":48832,"ancestor_record_offset":48248,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48884,"parent_record_offset":48832,"ancestor_record_offset":48248,"inlinee":27534,"source_line":46,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":48248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48728,"parent_record_offset":48700,"ancestor_record_offset":48248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48664,"parent_record_offset":48636,"ancestor_record_offset":48248,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":48444,"parent_record_offset":48372,"ancestor_record_offset":48248,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":48488,"parent_record_offset":48444,"ancestor_record_offset":48248,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":48968,"parent_record_offset":48284,"ancestor_record_offset":48248,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49136,"parent_record_offset":49012,"ancestor_record_offset":48248,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49012,"parent_record_offset":48968,"ancestor_record_offset":48248,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48700,"parent_record_offset":48372,"ancestor_record_offset":48248,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48764,"parent_record_offset":48372,"ancestor_record_offset":48248,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48636,"parent_record_offset":48372,"ancestor_record_offset":48248,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":49216,"parent_record_offset":49136,"ancestor_record_offset":48248,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":48372,"parent_record_offset":48284,"ancestor_record_offset":48248,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::polyline_length_between_coords
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BAA
// 首条位置: {"module":466,"inlinee":27562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":48832,"parent_record_offset":48284,"ancestor_record_offset":48284,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":48968,"parent_record_offset":48284,"ancestor_record_offset":48284,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":48372,"parent_record_offset":48284,"ancestor_record_offset":48284,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":49252,"parent_record_offset":49216,"ancestor_record_offset":48284,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48924,"parent_record_offset":48832,"ancestor_record_offset":48284,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":48884,"parent_record_offset":48832,"ancestor_record_offset":48284,"inlinee":27534,"source_line":46,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":48284,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48728,"parent_record_offset":48700,"ancestor_record_offset":48284,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":48664,"parent_record_offset":48636,"ancestor_record_offset":48284,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":48444,"parent_record_offset":48372,"ancestor_record_offset":48284,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":48488,"parent_record_offset":48444,"ancestor_record_offset":48284,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49136,"parent_record_offset":49012,"ancestor_record_offset":48284,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":49012,"parent_record_offset":48968,"ancestor_record_offset":48284,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48700,"parent_record_offset":48372,"ancestor_record_offset":48284,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48764,"parent_record_offset":48372,"ancestor_record_offset":48284,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":48636,"parent_record_offset":48372,"ancestor_record_offset":48284,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":49216,"parent_record_offset":49136,"ancestor_record_offset":48284,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::adjust_wooden_railing_endpoints
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BA0
// 首条位置: {"module":466,"inlinee":27552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":33656,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33752,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":34008,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":33944,"parent_record_offset":33916,"ancestor_record_offset":33612,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":33808,"parent_record_offset":33780,"ancestor_record_offset":33612,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":33684,"parent_record_offset":33656,"ancestor_record_offset":33612,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":33712,"parent_record_offset":33684,"ancestor_record_offset":33612,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33780,"parent_record_offset":33752,"ancestor_record_offset":33612,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33916,"parent_record_offset":33752,"ancestor_record_offset":33612,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side
// 记录形式: compiled_procedure; instances=1
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":33368,"rva":9137424,"va":5377846544}
// 编译实例 RVAs: 0x8b6d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2168527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e831e9deff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377847210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2168379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8c5e9deff","size":5,"soft_fail":false,"target_va":5375678736,"va":5377847110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-2168928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8a0e7deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377847627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9124910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8d2c374ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377847097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8607688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e838a87cff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377848291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4272563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e84dcebeff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377847326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141498980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12458792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8281bbe00","size":5,"soft_fail":false,"target_va":5390305664,"va":5377846867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414991f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12460318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e81e21be00","size":5,"soft_fail":false,"target_va":5390307824,"va":5377847501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12461978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e89a27be00","size":5,"soft_fail":false,"target_va":5390309472,"va":5377847489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a5520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12510737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e811e6be00","size":5,"soft_fail":false,"target_va":5390357792,"va":5377847050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39209800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8484b5602","size":5,"soft_fail":false,"target_va":5417057056,"va":5377847251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38773650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e892a34f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377847139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t38773669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8a5a34f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377848088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e874e3ffff","size":5,"soft_fail":false,"target_va":5377840496,"va":5377847799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846544,"raw_hex":"e8a4ceffff","size":5,"soft_fail":false,"target_va":5377835232,"va":5377847863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":35036,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":34772,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":35988,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":35572,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":34688,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":36084,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":34604,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::adjust_wooden_railing_endpoints
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":33612,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":27552,"source_line":315,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":35668,"parent_record_offset":33368,"ancestor_record_offset":33368,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":33368,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":33368,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":33368,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":33368,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":35336,"parent_record_offset":35312,"ancestor_record_offset":33368,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":33368,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":36344,"parent_record_offset":36320,"ancestor_record_offset":33368,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":35264,"parent_record_offset":35232,"ancestor_record_offset":33368,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":34904,"parent_record_offset":34872,"ancestor_record_offset":33368,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":36272,"parent_record_offset":36180,"ancestor_record_offset":33368,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":35172,"parent_record_offset":35140,"ancestor_record_offset":33368,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":34820,"parent_record_offset":34796,"ancestor_record_offset":33368,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":36208,"parent_record_offset":36180,"ancestor_record_offset":33368,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":36180,"parent_record_offset":36156,"ancestor_record_offset":33368,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":34872,"parent_record_offset":34844,"ancestor_record_offset":33368,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":35200,"parent_record_offset":35172,"ancestor_record_offset":33368,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":34844,"parent_record_offset":34820,"ancestor_record_offset":33368,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":36156,"parent_record_offset":36132,"ancestor_record_offset":33368,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":35068,"parent_record_offset":35036,"ancestor_record_offset":33368,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":35100,"parent_record_offset":35068,"ancestor_record_offset":33368,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":35448,"parent_record_offset":35100,"ancestor_record_offset":33368,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":35140,"parent_record_offset":35100,"ancestor_record_offset":33368,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":33656,"parent_record_offset":33612,"ancestor_record_offset":33368,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":34712,"parent_record_offset":34688,"ancestor_record_offset":33368,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":466,"record_offset":34796,"parent_record_offset":34772,"ancestor_record_offset":33368,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":466,"record_offset":34736,"parent_record_offset":34712,"ancestor_record_offset":33368,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":36240,"parent_record_offset":36208,"ancestor_record_offset":33368,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":33944,"parent_record_offset":33916,"ancestor_record_offset":33368,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":33808,"parent_record_offset":33780,"ancestor_record_offset":33368,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":35900,"parent_record_offset":35668,"ancestor_record_offset":33368,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":466,"record_offset":35392,"parent_record_offset":35100,"ancestor_record_offset":33368,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":35416,"parent_record_offset":35392,"ancestor_record_offset":33368,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":36108,"parent_record_offset":36084,"ancestor_record_offset":33368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":34652,"parent_record_offset":34628,"ancestor_record_offset":33368,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":35628,"parent_record_offset":35600,"ancestor_record_offset":33368,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":33684,"parent_record_offset":33656,"ancestor_record_offset":33368,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":36016,"parent_record_offset":35988,"ancestor_record_offset":33368,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":33712,"parent_record_offset":33684,"ancestor_record_offset":33368,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":36044,"parent_record_offset":36016,"ancestor_record_offset":33368,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":34628,"parent_record_offset":34604,"ancestor_record_offset":33368,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":35600,"parent_record_offset":35572,"ancestor_record_offset":33368,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33752,"parent_record_offset":33612,"ancestor_record_offset":33368,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33780,"parent_record_offset":33752,"ancestor_record_offset":33368,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":33916,"parent_record_offset":33752,"ancestor_record_offset":33368,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":34008,"parent_record_offset":33612,"ancestor_record_offset":33368,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":35716,"parent_record_offset":35668,"ancestor_record_offset":33368,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":35808,"parent_record_offset":35668,"ancestor_record_offset":33368,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 359
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x139A5
// 首条位置: {"module":1545,"inlinee":80293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells
// 记录形式: compiled_procedure; instances=1
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":9712,"rva":9126112,"va":5377835232}
// 编译实例 RVAs: 0x8b40e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2158543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e83110dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377837226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2156818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e8ee16dfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377835517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9115024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e870ea74ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377837211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408abb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e8626cffff","size":5,"soft_fail":false,"target_va":5377801024,"va":5377838809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140ad90
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11884593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e83158b500","size":5,"soft_fail":false,"target_va":5389725072,"va":5377840474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a1720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12503134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e85ec8be00","size":5,"soft_fail":false,"target_va":5390341920,"va":5377838781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38512793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e899a84b02","size":5,"soft_fail":false,"target_va":5416350944,"va":5377838146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38516386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e8a2b64b02","size":5,"soft_fail":false,"target_va":5416352064,"va":5377835673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38517203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e8d3b94b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5377835832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38781814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e876c34f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377838950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38781696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e800c34f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377839093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38782684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e8dcc64f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377838911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t38782767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377835232,"raw_hex":"e82fc74f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377838990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":9956,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":18568,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":16772,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":16736,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":9984,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":16708,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":10300,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":466,"record_offset":20492,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":27512,"source_line":325,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":17524,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":27541,"source_line":423,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":10332,"parent_record_offset":9712,"ancestor_record_offset":9712,"inlinee":27533,"source_line":446,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":9712,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15720,"parent_record_offset":15696,"ancestor_record_offset":9712,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":9712,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":9712,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15744,"parent_record_offset":15720,"ancestor_record_offset":9712,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":10244,"parent_record_offset":10220,"ancestor_record_offset":9712,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15672,"parent_record_offset":15640,"ancestor_record_offset":9712,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":10172,"parent_record_offset":10068,"ancestor_record_offset":9712,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":15640,"parent_record_offset":15612,"ancestor_record_offset":9712,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":15612,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":9712,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":9712,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":9712,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17752,"parent_record_offset":17720,"ancestor_record_offset":9712,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":10104,"parent_record_offset":10068,"ancestor_record_offset":9712,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":10068,"parent_record_offset":10040,"ancestor_record_offset":9712,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":9712,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":9712,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":9712,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17816,"parent_record_offset":17784,"ancestor_record_offset":9712,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17784,"parent_record_offset":17752,"ancestor_record_offset":9712,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":10040,"parent_record_offset":10012,"ancestor_record_offset":9712,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11084,"parent_record_offset":11048,"ancestor_record_offset":9712,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":12584,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":9712,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":18000,"parent_record_offset":17976,"ancestor_record_offset":9712,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":10492,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11016,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":9712,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11636,"parent_record_offset":11280,"ancestor_record_offset":9712,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11848,"parent_record_offset":11280,"ancestor_record_offset":9712,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17688,"parent_record_offset":17652,"ancestor_record_offset":9712,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17720,"parent_record_offset":17688,"ancestor_record_offset":9712,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":15204,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17588,"parent_record_offset":17556,"ancestor_record_offset":9712,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":20524,"parent_record_offset":20492,"ancestor_record_offset":9712,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11232,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":18440,"parent_record_offset":18404,"ancestor_record_offset":9712,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":466,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":9712,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":466,"record_offset":17976,"parent_record_offset":17652,"ancestor_record_offset":9712,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":466,"record_offset":17620,"parent_record_offset":17588,"ancestor_record_offset":9712,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":466,"record_offset":17652,"parent_record_offset":17620,"ancestor_record_offset":9712,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":9712,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":9712,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":11540,"parent_record_offset":11392,"ancestor_record_offset":9712,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":13952,"parent_record_offset":13916,"ancestor_record_offset":9712,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":466,"record_offset":11936,"parent_record_offset":11904,"ancestor_record_offset":9712,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":9712,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":9712,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":11876,"parent_record_offset":11280,"ancestor_record_offset":9712,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":11336,"parent_record_offset":11280,"ancestor_record_offset":9712,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":9712,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":13576,"parent_record_offset":13376,"ancestor_record_offset":9712,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":9712,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":9712,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11964,"parent_record_offset":11904,"ancestor_record_offset":9712,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11996,"parent_record_offset":11904,"ancestor_record_offset":9712,"inlinee":27534,"source_line":46,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":9712,"inlinee":11041,"source_line":60,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":9712,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":15120,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":466,"record_offset":13320,"parent_record_offset":13292,"ancestor_record_offset":9712,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":466,"record_offset":14040,"parent_record_offset":13576,"ancestor_record_offset":9712,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":466,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":9712,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":17556,"parent_record_offset":17524,"ancestor_record_offset":9712,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":9712,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":9712,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":18404,"parent_record_offset":18160,"ancestor_record_offset":9712,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":13292,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":9712,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":11608,"parent_record_offset":11280,"ancestor_record_offset":9712,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":10140,"parent_record_offset":10104,"ancestor_record_offset":9712,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10952,"parent_record_offset":10920,"ancestor_record_offset":9712,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":9712,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10772,"parent_record_offset":10744,"ancestor_record_offset":9712,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":9712,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":13256,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":466,"record_offset":11500,"parent_record_offset":11476,"ancestor_record_offset":9712,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":13196,"parent_record_offset":12584,"ancestor_record_offset":9712,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":10012,"parent_record_offset":9984,"ancestor_record_offset":9712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":12980,"parent_record_offset":12584,"ancestor_record_offset":9712,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":9712,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":13540,"parent_record_offset":13464,"ancestor_record_offset":9712,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":9712,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":13508,"parent_record_offset":13464,"ancestor_record_offset":9712,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":11196,"parent_record_offset":11048,"ancestor_record_offset":9712,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":13228,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":10572,"parent_record_offset":10520,"ancestor_record_offset":9712,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":9712,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":18072,"parent_record_offset":18048,"ancestor_record_offset":9712,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":13464,"parent_record_offset":13428,"ancestor_record_offset":9712,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":12024,"parent_record_offset":11996,"ancestor_record_offset":9712,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":11428,"parent_record_offset":11392,"ancestor_record_offset":9712,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13664,"parent_record_offset":13620,"ancestor_record_offset":9712,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13832,"parent_record_offset":13708,"ancestor_record_offset":9712,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13708,"parent_record_offset":13664,"ancestor_record_offset":9712,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":14660,"parent_record_offset":14628,"ancestor_record_offset":9712,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":9712,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12908,"parent_record_offset":12820,"ancestor_record_offset":9712,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10832,"parent_record_offset":10520,"ancestor_record_offset":9712,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10920,"parent_record_offset":10520,"ancestor_record_offset":9712,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10744,"parent_record_offset":10520,"ancestor_record_offset":9712,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":14396,"parent_record_offset":14240,"ancestor_record_offset":9712,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":13428,"parent_record_offset":13376,"ancestor_record_offset":9712,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":466,"record_offset":14980,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":13916,"parent_record_offset":13832,"ancestor_record_offset":9712,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":16928,"parent_record_offset":16888,"ancestor_record_offset":9712,"inlinee":27540,"source_line":420,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":18192,"parent_record_offset":18160,"ancestor_record_offset":9712,"inlinee":27543,"source_line":426,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":12768,"parent_record_offset":12584,"ancestor_record_offset":9712,"inlinee":27536,"source_line":461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":13620,"parent_record_offset":13576,"ancestor_record_offset":9712,"inlinee":27537,"source_line":472,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":14628,"parent_record_offset":14596,"ancestor_record_offset":9712,"inlinee":27538,"source_line":488,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":10644,"parent_record_offset":10520,"ancestor_record_offset":9712,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":10520,"parent_record_offset":10332,"ancestor_record_offset":9712,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B94
// 首条位置: {"module":466,"inlinee":27540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":16968,"parent_record_offset":16928,"ancestor_record_offset":16928,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":17268,"parent_record_offset":17232,"ancestor_record_offset":16928,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":17080,"parent_record_offset":17008,"ancestor_record_offset":16928,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":17152,"parent_record_offset":17008,"ancestor_record_offset":16928,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":17008,"parent_record_offset":16968,"ancestor_record_offset":16928,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":17232,"parent_record_offset":17152,"ancestor_record_offset":16928,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B95
// 首条位置: {"module":466,"inlinee":27541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":17556,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":17524,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":17524,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":17524,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":17844,"parent_record_offset":17816,"ancestor_record_offset":17524,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17752,"parent_record_offset":17720,"ancestor_record_offset":17524,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17816,"parent_record_offset":17784,"ancestor_record_offset":17524,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":17784,"parent_record_offset":17752,"ancestor_record_offset":17524,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":18000,"parent_record_offset":17976,"ancestor_record_offset":17524,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17688,"parent_record_offset":17652,"ancestor_record_offset":17524,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17720,"parent_record_offset":17688,"ancestor_record_offset":17524,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":17588,"parent_record_offset":17556,"ancestor_record_offset":17524,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":18440,"parent_record_offset":18404,"ancestor_record_offset":17524,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":466,"record_offset":17976,"parent_record_offset":17652,"ancestor_record_offset":17524,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":466,"record_offset":17620,"parent_record_offset":17588,"ancestor_record_offset":17524,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":466,"record_offset":17652,"parent_record_offset":17620,"ancestor_record_offset":17524,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":17524,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":18160,"parent_record_offset":18072,"ancestor_record_offset":17524,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":17524,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":18404,"parent_record_offset":18160,"ancestor_record_offset":17524,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":18288,"parent_record_offset":18228,"ancestor_record_offset":17524,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":18128,"parent_record_offset":18072,"ancestor_record_offset":17524,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":18476,"parent_record_offset":18440,"ancestor_record_offset":17524,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":18072,"parent_record_offset":18048,"ancestor_record_offset":17524,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":18228,"parent_record_offset":18192,"ancestor_record_offset":17524,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":18192,"parent_record_offset":18160,"ancestor_record_offset":17524,"inlinee":27543,"source_line":426,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 426
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B97
// 首条位置: {"module":466,"inlinee":27543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":18228,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":18288,"parent_record_offset":18228,"ancestor_record_offset":18192,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 446
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B8D
// 首条位置: {"module":466,"inlinee":27533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":15612,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":12584,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":10492,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11016,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":15204,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11232,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":15120,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":13292,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":13256,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":15956,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":13228,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":15396,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":466,"record_offset":14980,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":10520,"parent_record_offset":10332,"ancestor_record_offset":10332,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":10332,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15720,"parent_record_offset":15696,"ancestor_record_offset":10332,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":10332,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":16164,"parent_record_offset":16140,"ancestor_record_offset":10332,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15744,"parent_record_offset":15720,"ancestor_record_offset":10332,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":10332,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":15672,"parent_record_offset":15640,"ancestor_record_offset":10332,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":16116,"parent_record_offset":16028,"ancestor_record_offset":10332,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":15640,"parent_record_offset":15612,"ancestor_record_offset":10332,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":10332,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":10332,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":10332,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":10332,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":10332,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":10332,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":10332,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":10332,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":10332,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11084,"parent_record_offset":11048,"ancestor_record_offset":10332,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":10332,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":10332,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11636,"parent_record_offset":11280,"ancestor_record_offset":10332,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":11848,"parent_record_offset":11280,"ancestor_record_offset":10332,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":466,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":10332,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":10332,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":10332,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":466,"record_offset":11540,"parent_record_offset":11392,"ancestor_record_offset":10332,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":13952,"parent_record_offset":13916,"ancestor_record_offset":10332,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":466,"record_offset":11936,"parent_record_offset":11904,"ancestor_record_offset":10332,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":10332,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":10332,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":11876,"parent_record_offset":11280,"ancestor_record_offset":10332,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":466,"record_offset":11336,"parent_record_offset":11280,"ancestor_record_offset":10332,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":10332,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":466,"record_offset":13576,"parent_record_offset":13376,"ancestor_record_offset":10332,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":10332,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":466,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":10332,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11964,"parent_record_offset":11904,"ancestor_record_offset":10332,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11996,"parent_record_offset":11904,"ancestor_record_offset":10332,"inlinee":27534,"source_line":46,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":466,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":10332,"inlinee":11041,"source_line":60,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":466,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":10332,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":466,"record_offset":13320,"parent_record_offset":13292,"ancestor_record_offset":10332,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":466,"record_offset":14040,"parent_record_offset":13576,"ancestor_record_offset":10332,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":10332,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":10332,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":11608,"parent_record_offset":11280,"ancestor_record_offset":10332,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":16084,"parent_record_offset":16056,"ancestor_record_offset":10332,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10952,"parent_record_offset":10920,"ancestor_record_offset":10332,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":10332,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":10772,"parent_record_offset":10744,"ancestor_record_offset":10332,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":10332,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":466,"record_offset":11500,"parent_record_offset":11476,"ancestor_record_offset":10332,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":13196,"parent_record_offset":12584,"ancestor_record_offset":10332,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":10332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":12980,"parent_record_offset":12584,"ancestor_record_offset":10332,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":10332,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":13540,"parent_record_offset":13464,"ancestor_record_offset":10332,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":10332,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":13508,"parent_record_offset":13464,"ancestor_record_offset":10332,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":11196,"parent_record_offset":11048,"ancestor_record_offset":10332,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":10572,"parent_record_offset":10520,"ancestor_record_offset":10332,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":10332,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":13464,"parent_record_offset":13428,"ancestor_record_offset":10332,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":12024,"parent_record_offset":11996,"ancestor_record_offset":10332,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":11428,"parent_record_offset":11392,"ancestor_record_offset":10332,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13664,"parent_record_offset":13620,"ancestor_record_offset":10332,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13832,"parent_record_offset":13708,"ancestor_record_offset":10332,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13708,"parent_record_offset":13664,"ancestor_record_offset":10332,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":14660,"parent_record_offset":14628,"ancestor_record_offset":10332,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":10332,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12908,"parent_record_offset":12820,"ancestor_record_offset":10332,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10832,"parent_record_offset":10520,"ancestor_record_offset":10332,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10920,"parent_record_offset":10520,"ancestor_record_offset":10332,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":10744,"parent_record_offset":10520,"ancestor_record_offset":10332,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":14396,"parent_record_offset":14240,"ancestor_record_offset":10332,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":13428,"parent_record_offset":13376,"ancestor_record_offset":10332,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":13916,"parent_record_offset":13832,"ancestor_record_offset":10332,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":12768,"parent_record_offset":12584,"ancestor_record_offset":10332,"inlinee":27536,"source_line":461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":13620,"parent_record_offset":13576,"ancestor_record_offset":10332,"inlinee":27537,"source_line":472,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":14628,"parent_record_offset":14596,"ancestor_record_offset":10332,"inlinee":27538,"source_line":488,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":466,"record_offset":10644,"parent_record_offset":10520,"ancestor_record_offset":10332,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 461
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B90
// 首条位置: {"module":466,"inlinee":27536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":12768,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":12852,"parent_record_offset":12820,"ancestor_record_offset":12768,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12908,"parent_record_offset":12820,"ancestor_record_offset":12768,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":12768,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B91
// 首条位置: {"module":466,"inlinee":27537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13664,"parent_record_offset":13620,"ancestor_record_offset":13620,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":13952,"parent_record_offset":13916,"ancestor_record_offset":13620,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":13788,"parent_record_offset":13708,"ancestor_record_offset":13620,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13832,"parent_record_offset":13708,"ancestor_record_offset":13620,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":13708,"parent_record_offset":13664,"ancestor_record_offset":13620,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":13916,"parent_record_offset":13832,"ancestor_record_offset":13620,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B92
// 首条位置: {"module":466,"inlinee":27538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":14660,"parent_record_offset":14628,"ancestor_record_offset":14628,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":14880,"parent_record_offset":14852,"ancestor_record_offset":14628,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":14740,"parent_record_offset":14692,"ancestor_record_offset":14628,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":14804,"parent_record_offset":14692,"ancestor_record_offset":14628,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":14692,"parent_record_offset":14660,"ancestor_record_offset":14628,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":14852,"parent_record_offset":14804,"ancestor_record_offset":14628,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at
// 记录形式: compiled_procedure; instances=1
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":25944,"rva":9134096,"va":5377843216}
// 编译实例 RVAs: 0x8b6010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2164913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e84ff7deff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377843596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2165191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e839f6deff","size":5,"soft_fail":false,"target_va":5375678736,"va":5377843922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2165081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8a7f6deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377843780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9121394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e88ed174ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377843581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8605831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e879af7cff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377846434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4268903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e899dcbeff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377843666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b3fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e857d6ffff","size":5,"soft_fail":false,"target_va":5377834976,"va":5377845636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t282168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8384e0400","size":5,"soft_fail":false,"target_va":5378126944,"va":5377844771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3881788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e83c3b3b00","size":5,"soft_fail":false,"target_va":5381726576,"va":5377844783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141498980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12461912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e85827be00","size":5,"soft_fail":false,"target_va":5390305664,"va":5377843747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b6a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12583651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8e302c000","size":5,"soft_fail":false,"target_va":5390428736,"va":5377845080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38775001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8d9a84f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377845763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38774990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8cea84f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377845799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38775974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e8a6ac4f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377845783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5538949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e885845400","size":5,"soft_fail":false,"target_va":5383383152,"va":5377844198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843216,"raw_hex":"e832f2ffff","size":5,"soft_fail":false,"target_va":5377840496,"va":5377844025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":26916,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":26732,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":27688,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":32208,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":466,"record_offset":27000,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":466,"record_offset":26300,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":26676,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":26188,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_straight_wooden_railing
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":27968,"parent_record_offset":25944,"ancestor_record_offset":25944,"inlinee":27551,"source_line":602,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26816,"parent_record_offset":26792,"ancestor_record_offset":25944,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26840,"parent_record_offset":26816,"ancestor_record_offset":25944,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":25944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":25944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26864,"parent_record_offset":26840,"ancestor_record_offset":25944,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27908,"parent_record_offset":27884,"ancestor_record_offset":25944,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":25944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26792,"parent_record_offset":26760,"ancestor_record_offset":25944,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27836,"parent_record_offset":27804,"ancestor_record_offset":25944,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26540,"parent_record_offset":26444,"ancestor_record_offset":25944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":466,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":25944,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27744,"parent_record_offset":27716,"ancestor_record_offset":25944,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26476,"parent_record_offset":26444,"ancestor_record_offset":25944,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26444,"parent_record_offset":26416,"ancestor_record_offset":25944,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27804,"parent_record_offset":27772,"ancestor_record_offset":25944,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27772,"parent_record_offset":27744,"ancestor_record_offset":25944,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26416,"parent_record_offset":26388,"ancestor_record_offset":25944,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":466,"record_offset":26940,"parent_record_offset":26916,"ancestor_record_offset":25944,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":26964,"parent_record_offset":26940,"ancestor_record_offset":25944,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":466,"record_offset":27716,"parent_record_offset":27688,"ancestor_record_offset":25944,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30228,"parent_record_offset":30196,"ancestor_record_offset":25944,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":25944,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30264,"parent_record_offset":30196,"ancestor_record_offset":25944,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":466,"record_offset":30508,"parent_record_offset":30072,"ancestor_record_offset":25944,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":25944,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":28372,"parent_record_offset":28348,"ancestor_record_offset":25944,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":25944,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":28204,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":28444,"parent_record_offset":28420,"ancestor_record_offset":25944,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":466,"record_offset":30312,"parent_record_offset":30288,"ancestor_record_offset":25944,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":26508,"parent_record_offset":26476,"ancestor_record_offset":25944,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":30356,"parent_record_offset":30132,"ancestor_record_offset":25944,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":30436,"parent_record_offset":30408,"ancestor_record_offset":25944,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28632,"parent_record_offset":28608,"ancestor_record_offset":25944,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28876,"parent_record_offset":28844,"ancestor_record_offset":25944,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28784,"parent_record_offset":28756,"ancestor_record_offset":25944,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":28396,"parent_record_offset":28372,"ancestor_record_offset":25944,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":26360,"parent_record_offset":26332,"ancestor_record_offset":25944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":25944,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":28316,"parent_record_offset":28260,"ancestor_record_offset":25944,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":26700,"parent_record_offset":26676,"ancestor_record_offset":25944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":26212,"parent_record_offset":26188,"ancestor_record_offset":25944,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":25944,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":28260,"parent_record_offset":28228,"ancestor_record_offset":25944,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30132,"parent_record_offset":30108,"ancestor_record_offset":25944,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30172,"parent_record_offset":30132,"ancestor_record_offset":25944,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30196,"parent_record_offset":30172,"ancestor_record_offset":25944,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30108,"parent_record_offset":30072,"ancestor_record_offset":25944,"inlinee":26746,"source_line":237,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29480,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29316,"parent_record_offset":29280,"ancestor_record_offset":25944,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29280,"parent_record_offset":29244,"ancestor_record_offset":25944,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29244,"parent_record_offset":29212,"ancestor_record_offset":25944,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29212,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28844,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28608,"parent_record_offset":28584,"ancestor_record_offset":25944,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$37::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28584,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":26420,"source_line":1617,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29120,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28756,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":27532,"source_line":1781,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":28348,"parent_record_offset":28228,"ancestor_record_offset":25944,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":28228,"parent_record_offset":28204,"ancestor_record_offset":25944,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":32056,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":29352,"parent_record_offset":29316,"ancestor_record_offset":25944,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":27032,"parent_record_offset":27000,"ancestor_record_offset":25944,"inlinee":27549,"source_line":515,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":26332,"parent_record_offset":26300,"ancestor_record_offset":25944,"inlinee":27550,"source_line":518,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":30072,"parent_record_offset":27968,"ancestor_record_offset":25944,"inlinee":26745,"source_line":825,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 515
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B9D
// 首条位置: {"module":466,"inlinee":27549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":27068,"parent_record_offset":27032,"ancestor_record_offset":27032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27280,"parent_record_offset":27256,"ancestor_record_offset":27032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27304,"parent_record_offset":27280,"ancestor_record_offset":27032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":27032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":27256,"parent_record_offset":27164,"ancestor_record_offset":27032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27192,"parent_record_offset":27164,"ancestor_record_offset":27032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27164,"parent_record_offset":27140,"ancestor_record_offset":27032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":27140,"parent_record_offset":27116,"ancestor_record_offset":27032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":27224,"parent_record_offset":27192,"ancestor_record_offset":27032,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":27032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B9E
// 首条位置: {"module":466,"inlinee":27550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":26360,"parent_record_offset":26332,"ancestor_record_offset":26332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":26332,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":26332,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":26332,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":466,"record_offset":26540,"parent_record_offset":26444,"ancestor_record_offset":26332,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26476,"parent_record_offset":26444,"ancestor_record_offset":26332,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26444,"parent_record_offset":26416,"ancestor_record_offset":26332,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":466,"record_offset":26416,"parent_record_offset":26388,"ancestor_record_offset":26332,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":26508,"parent_record_offset":26476,"ancestor_record_offset":26332,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":466,"record_offset":26388,"parent_record_offset":26360,"ancestor_record_offset":26332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::add_corner_wooden_railing_piece
// 记录形式: compiled_procedure; instances=1
// 源码行: 537
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":24208,"rva":9133152,"va":5377842272}
// 编译实例 RVAs: 0x8b5c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5482423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377842272,"raw_hex":"e8b7a75300","size":5,"soft_fail":false,"target_va":5383325024,"va":5377842596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38778609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377842272,"raw_hex":"e8f1b64f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377843148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377842272,"raw_hex":"e813deffff","size":5,"soft_fail":false,"target_va":5377834256,"va":5377842936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24444,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24504,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24476,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":24932,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":24836,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":25112,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":25384,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":24648,"parent_record_offset":24624,"ancestor_record_offset":24208,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":25140,"parent_record_offset":25112,"ancestor_record_offset":24208,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":24688,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":25416,"parent_record_offset":25384,"ancestor_record_offset":24208,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24592,"parent_record_offset":24560,"ancestor_record_offset":24208,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24560,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24528,"parent_record_offset":24504,"ancestor_record_offset":24208,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":24776,"parent_record_offset":24752,"ancestor_record_offset":24208,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":24624,"parent_record_offset":24592,"ancestor_record_offset":24208,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_straight_wooden_railing
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 602
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B9F
// 首条位置: {"module":466,"inlinee":27551}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":28204,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":28120,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29480,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29212,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28844,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29028,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$37::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28584,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":26420,"source_line":1617,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29120,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28756,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":27532,"source_line":1781,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":32056,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":30072,"parent_record_offset":27968,"ancestor_record_offset":27968,"inlinee":26745,"source_line":825,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30228,"parent_record_offset":30196,"ancestor_record_offset":27968,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":27968,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":30264,"parent_record_offset":30196,"ancestor_record_offset":27968,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":466,"record_offset":30508,"parent_record_offset":30072,"ancestor_record_offset":27968,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":27968,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":28372,"parent_record_offset":28348,"ancestor_record_offset":27968,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":466,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":27968,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":28444,"parent_record_offset":28420,"ancestor_record_offset":27968,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":466,"record_offset":30312,"parent_record_offset":30288,"ancestor_record_offset":27968,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":30356,"parent_record_offset":30132,"ancestor_record_offset":27968,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":30436,"parent_record_offset":30408,"ancestor_record_offset":27968,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28632,"parent_record_offset":28608,"ancestor_record_offset":27968,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28876,"parent_record_offset":28844,"ancestor_record_offset":27968,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":28784,"parent_record_offset":28756,"ancestor_record_offset":27968,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":28396,"parent_record_offset":28372,"ancestor_record_offset":27968,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":27968,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":28316,"parent_record_offset":28260,"ancestor_record_offset":27968,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":28288,"parent_record_offset":28260,"ancestor_record_offset":27968,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":466,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":27968,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":28260,"parent_record_offset":28228,"ancestor_record_offset":27968,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30132,"parent_record_offset":30108,"ancestor_record_offset":27968,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30172,"parent_record_offset":30132,"ancestor_record_offset":27968,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30196,"parent_record_offset":30172,"ancestor_record_offset":27968,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":466,"record_offset":30108,"parent_record_offset":30072,"ancestor_record_offset":27968,"inlinee":26746,"source_line":237,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29316,"parent_record_offset":29280,"ancestor_record_offset":27968,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29280,"parent_record_offset":29244,"ancestor_record_offset":27968,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":29244,"parent_record_offset":29212,"ancestor_record_offset":27968,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":28608,"parent_record_offset":28584,"ancestor_record_offset":27968,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":28348,"parent_record_offset":28228,"ancestor_record_offset":27968,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":466,"record_offset":28228,"parent_record_offset":28204,"ancestor_record_offset":27968,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":29352,"parent_record_offset":29316,"ancestor_record_offset":27968,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts
// 记录形式: compiled_procedure; instances=1
// 源码行: 682
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":20960,"rva":9131376,"va":5377840496}
// 编译实例 RVAs: 0x8b5570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b3fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840496,"raw_hex":"e821e4ffff","size":5,"soft_fail":false,"target_va":5377834976,"va":5377842106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5483572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840496,"raw_hex":"e834ac5300","size":5,"soft_fail":false,"target_va":5383325024,"va":5377841447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38779515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840496,"raw_hex":"e87bba4f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377842242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840496,"raw_hex":"e887e3ffff","size":5,"soft_fail":false,"target_va":5377834256,"va":5377841540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":466,"record_offset":21224,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":22280,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":22244,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":21688,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":466,"record_offset":21836,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":466,"record_offset":21608,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22492,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22020,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22740,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":22768,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":22884,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$37::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":21488,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":26420,"source_line":1617,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":21432,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_support
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":22972,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":27547,"source_line":799,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":22192,"parent_record_offset":22168,"ancestor_record_offset":20960,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":21896,"parent_record_offset":21868,"ancestor_record_offset":20960,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":21544,"parent_record_offset":21516,"ancestor_record_offset":20960,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":22676,"parent_record_offset":22516,"ancestor_record_offset":20960,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":23288,"parent_record_offset":23260,"ancestor_record_offset":20960,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":21456,"parent_record_offset":21432,"ancestor_record_offset":20960,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":21384,"parent_record_offset":21272,"ancestor_record_offset":20960,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":466,"record_offset":21336,"parent_record_offset":21272,"ancestor_record_offset":20960,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":466,"record_offset":21644,"parent_record_offset":21608,"ancestor_record_offset":20960,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":466,"record_offset":21272,"parent_record_offset":21224,"ancestor_record_offset":20960,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22136,"parent_record_offset":22104,"ancestor_record_offset":20960,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":20960,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22080,"parent_record_offset":22048,"ancestor_record_offset":20960,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":22048,"parent_record_offset":22020,"ancestor_record_offset":20960,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":21740,"parent_record_offset":21716,"ancestor_record_offset":20960,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23420,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":20960,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23140,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":20960,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23456,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23348,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":21516,"parent_record_offset":21488,"ancestor_record_offset":20960,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23052,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23260,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":24052,"parent_record_offset":22972,"ancestor_record_offset":20960,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":22168,"parent_record_offset":22136,"ancestor_record_offset":20960,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":21716,"parent_record_offset":21688,"ancestor_record_offset":20960,"inlinee":27545,"source_line":704,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":21868,"parent_record_offset":21836,"ancestor_record_offset":20960,"inlinee":27546,"source_line":705,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 704
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B99
// 首条位置: {"module":466,"inlinee":27545}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":21740,"parent_record_offset":21716,"ancestor_record_offset":21716,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":21768,"parent_record_offset":21740,"ancestor_record_offset":21716,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 705
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B9A
// 首条位置: {"module":466,"inlinee":27546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":466,"record_offset":21896,"parent_record_offset":21868,"ancestor_record_offset":21868,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":466,"record_offset":21924,"parent_record_offset":21868,"ancestor_record_offset":21868,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":21952,"parent_record_offset":21924,"ancestor_record_offset":21868,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post
// 记录形式: compiled_procedure; instances=1
// 源码行: 743
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":466,"record_offset":7324,"rva":9125136,"va":5377834256}
// 编译实例 RVAs: 0x8b3d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b3fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t73","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377834256,"raw_hex":"e849000000","size":5,"soft_fail":false,"target_va":5377834976,"va":5377834898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5548747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377834256,"raw_hex":"e8cbaa5400","size":5,"soft_fail":false,"target_va":5383383152,"va":5377834400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":7628,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":7536,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":27532,"source_line":1781,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post_with_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":466,"record_offset":7812,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":26744,"source_line":774,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7324,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":8072,"parent_record_offset":8040,"ancestor_record_offset":7324,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":7324,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":7568,"parent_record_offset":7536,"ancestor_record_offset":7324,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8164,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8292,"parent_record_offset":8260,"ancestor_record_offset":7324,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8260,"parent_record_offset":8228,"ancestor_record_offset":7324,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":7324,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8204,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":7888,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8040,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":7944,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":8452,"parent_record_offset":7812,"ancestor_record_offset":7324,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":8324,"parent_record_offset":8292,"ancestor_record_offset":7324,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post_with_height
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 774
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6878
// 首条位置: {"module":432,"inlinee":26744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8164,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8204,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":7888,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8040,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":13748,"parent_record_offset":13720,"ancestor_record_offset":13720,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":8452,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":432,"record_offset":13804,"parent_record_offset":13720,"ancestor_record_offset":13720,"inlinee":26745,"source_line":825,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":432,"record_offset":14152,"parent_record_offset":13804,"ancestor_record_offset":13720,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7812,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":14000,"parent_record_offset":13864,"ancestor_record_offset":13720,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":13964,"parent_record_offset":13932,"ancestor_record_offset":13720,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":8072,"parent_record_offset":8040,"ancestor_record_offset":7812,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":7812,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":13720,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13720,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13908,"parent_record_offset":13864,"ancestor_record_offset":13720,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":13720,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":13720,"inlinee":26746,"source_line":237,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8292,"parent_record_offset":8260,"ancestor_record_offset":7812,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8260,"parent_record_offset":8228,"ancestor_record_offset":7812,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":7812,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":8324,"parent_record_offset":8292,"ancestor_record_offset":7812,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_support
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 799
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B9B
// 首条位置: {"module":466,"inlinee":27547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23420,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23140,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23456,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23348,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":24144,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23052,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23260,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":466,"record_offset":24052,"parent_record_offset":22972,"ancestor_record_offset":22972,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":466,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":22972,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":24168,"parent_record_offset":24144,"ancestor_record_offset":22972,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":23380,"parent_record_offset":23348,"ancestor_record_offset":22972,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":23288,"parent_record_offset":23260,"ancestor_record_offset":22972,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":466,"record_offset":23080,"parent_record_offset":23052,"ancestor_record_offset":22972,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":22972,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23512,"parent_record_offset":23480,"ancestor_record_offset":22972,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":466,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":22972,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":466,"record_offset":23196,"parent_record_offset":23164,"ancestor_record_offset":22972,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 825
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6879
// 首条位置: {"module":432,"inlinee":26745}
// 编译实例 RVAs: 0x8b3fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t291817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377834976,"raw_hex":"e8e9730400","size":5,"soft_fail":false,"target_va":5378126944,"va":5377835122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3891437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377834976,"raw_hex":"e8ed603b00","size":5,"soft_fail":false,"target_va":5381726576,"va":5377835134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b6a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_wooden_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12593520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377834976,"raw_hex":"e87029c000","size":5,"soft_fail":false,"target_va":5390428736,"va":5377835211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":432,"record_offset":14152,"parent_record_offset":13804,"ancestor_record_offset":13804,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":13804,"inlinee":26746,"source_line":237,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":9400,"parent_record_offset":9368,"ancestor_record_offset":9072,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":9460,"parent_record_offset":9436,"ancestor_record_offset":9072,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":466,"record_offset":9436,"parent_record_offset":9368,"ancestor_record_offset":9072,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":466,"record_offset":9484,"parent_record_offset":9460,"ancestor_record_offset":9072,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":14000,"parent_record_offset":13864,"ancestor_record_offset":13804,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":13964,"parent_record_offset":13932,"ancestor_record_offset":13804,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13804,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13908,"parent_record_offset":13864,"ancestor_record_offset":13804,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":13804,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00432.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_wooden_railings_assemble.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::WoodenRailingSpan,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_wooden_railings_assemble.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_wooden_railings_assemble.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::add_corner_wooden_railing_piece
// 信息 ID: 11c579337f8086266fa854d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36ad21 -> 0x8b5c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37da77 -> 0x8b5c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::adjust_wooden_railing_endpoints
// 信息 ID: 140acaa86edf4f15967b2571
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side; module=466, inline_record=33612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings
// 信息 ID: 1c1d72be9193f56eb2431eec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36f95a -> 0x8b7530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x3823bc -> 0x8b7530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0
// 信息 ID: 2faed55208a4d51780da5c72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings; module=466, inline_record=39652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$0::closure$0
// 信息 ID: d603596c5aa8cfcde9ab62b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings; module=466, inline_record=40132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings::closure$2
// 信息 ID: 82c12126ba5176960c0c5c48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings; module=466, inline_record=48248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side
// 信息 ID: ea950034f4df2dd160404d03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings @ 0x8b0c8a -> 0x8b6d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side::closure$0
// 信息 ID: 958aeeea8074219b0e440d9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemantic3D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemantic3D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side::closure_env$0> >; module=1545, inline_record=66320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at
// 信息 ID: 2689d221daa25eae922adf8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36b5a6 -> 0x8b6010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37daa8 -> 0x8b6010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips @ 0x8b1ebf -> 0x8b6010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$0
// 信息 ID: 0bde8c3c807bc003d1015a76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at; module=466, inline_record=27032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at::closure$1
// 信息 ID: f64b9049eb659bb9921cf381
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at; module=466, inline_record=26332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings
// 信息 ID: 074e7974ee12ce4193cd13cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x369efb -> 0x8b2c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37d5ae -> 0x8b2c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$0
// 信息 ID: a3291dccad9a234c1ee55372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=1340; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=1456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$1
// 信息 ID: c21323abc3d3a9d862509996
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$3> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure_env$1> > > >; module=1600, inline_record=33548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings::closure$2
// 信息 ID: b1c687ea96511d4d9abc8ee9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=2868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_straight_wooden_railing
// 信息 ID: ab91656d1e32200e9431bad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at; module=466, inline_record=27968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts
// 信息 ID: c80258188fcca78383f33c77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at @ 0x8b6339 -> 0x8b5570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side @ 0x8b71f7 -> 0x8b5570 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b8d09 -> 0x8b5570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$1
// 信息 ID: 9a573b66279fcfcd2d48e566
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts; module=466, inline_record=21716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts::closure$2
// 信息 ID: 7da1da455b2540430c25b3f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts; module=466, inline_record=21868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::polyline_length_between_coords
// 信息 ID: 0534fc7c10c10f6af86f618e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings; module=466, inline_record=48284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
// 信息 ID: 04cdd8af400c34bf62b7713a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post @ 0x8b3f92 -> 0x8b3fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts @ 0x8b5bba -> 0x8b3fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at @ 0x8b6984 -> 0x8b3fe0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=13804; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at; module=466, inline_record=30072; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post
// 信息 ID: 589823a69deb49248691b7b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts @ 0x8b5984 -> 0x8b3d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::add_corner_wooden_railing_piece @ 0x8b5ef8 -> 0x8b3d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::add_corner_wooden_railing_piece @ 0x8b5f6e -> 0x8b3d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post_with_height
// 信息 ID: e57cfc6e573bc4a01f1aae5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=13720; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post; module=466, inline_record=7812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_support
// 信息 ID: a5b3b005c3a705e608b561ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::construct_wooden_railing_posts; module=466, inline_record=22972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells
// 信息 ID: cb28134410fbc320522adea1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side @ 0x8b7237 -> 0x8b40e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b8d69 -> 0x8b40e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$0
// 信息 ID: 7ccccfeea377a72434ce1083
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=16928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1
// 信息 ID: bd1bab9fc53d61d260c8ca80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=17524; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=19028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells::closure$1::closure$0
// 信息 ID: ad23310472501504416d1d0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=18192; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=19684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path
// 信息 ID: b9767bd084b2695280521e39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=10332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$0
// 信息 ID: 7076390224130249eaf4e771
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=12768; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=13008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$1
// 信息 ID: 49ca3f9d638d5b44c75e6051
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=13620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path::closure$2
// 信息 ID: e0614f2b1bd53b3d88c83fe1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::railing_cell_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=14628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
