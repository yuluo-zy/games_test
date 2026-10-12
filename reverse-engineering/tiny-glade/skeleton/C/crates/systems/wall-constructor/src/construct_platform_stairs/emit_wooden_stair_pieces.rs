// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1768396f4de3fbcdf7b76136c1ac19a46037c319cf361b0dab62d39fe68d3269
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: emit_wooden_stair_pieces.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 395, 1472, 1487, 3140
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
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
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x660C
// 首条位置: {"module":395,"inlinee":26124}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$4::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136BE
// 首条位置: {"module":1472,"inlinee":79550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8784,"parent_record_offset":8752,"ancestor_record_offset":8752,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":8752,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8752,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":8752,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9196,"parent_record_offset":9016,"ancestor_record_offset":8752,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":8752,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9016,"parent_record_offset":8984,"ancestor_record_offset":8752,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":8752,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9052,"parent_record_offset":9016,"ancestor_record_offset":8752,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":8884,"parent_record_offset":8816,"ancestor_record_offset":8752,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8856,"parent_record_offset":8816,"ancestor_record_offset":8752,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8928,"parent_record_offset":8884,"ancestor_record_offset":8752,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":8752,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1472,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8752,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1472,"record_offset":9156,"parent_record_offset":9100,"ancestor_record_offset":8752,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8752,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9128,"parent_record_offset":9100,"ancestor_record_offset":8752,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":9324,"parent_record_offset":8884,"ancestor_record_offset":8752,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":8752,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_pieces
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":8520,"rva":38651088,"va":5407360208}
// 编译实例 RVAs: 0x24dc4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-31681661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360208,"raw_hex":"e883931cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407360360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360208,"raw_hex":"e820f1ffff","size":5,"soft_fail":false,"target_va":5407356528,"va":5407360331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::wooden_stair_layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360208,"raw_hex":"e88decffff","size":5,"soft_fail":false,"target_va":5407355280,"va":5407360254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":8708,"parent_record_offset":8520,"ancestor_record_offset":8520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":8520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":8520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":8520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":8896,"parent_record_offset":8804,"ancestor_record_offset":8520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":8832,"parent_record_offset":8804,"ancestor_record_offset":8520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":8520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":8780,"parent_record_offset":8756,"ancestor_record_offset":8520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":8864,"parent_record_offset":8832,"ancestor_record_offset":8520,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":8520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":3476,"rva":38647408,"va":5407356528}
// 编译实例 RVAs: 0x24db670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24033942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e86a4591fe","size":5,"soft_fail":false,"target_va":5383325024,"va":5407358961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24032388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e87c4b91fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407357439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9699738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e89a019400","size":5,"soft_fail":false,"target_va":5417057840,"va":5407358097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9022778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e83aad8900","size":5,"soft_fail":false,"target_va":5416380336,"va":5407357553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9262079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8ff538d00","size":5,"soft_fail":false,"target_va":5416622208,"va":5407360124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9262046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8de538d00","size":5,"soft_fail":false,"target_va":5416622240,"va":5407360189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27449101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8f3285dfe","size":5,"soft_fail":false,"target_va":5379908400,"va":5407357496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27449673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8b7265dfe","size":5,"soft_fail":false,"target_va":5379907856,"va":5407357524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27448523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8352b5dfe","size":5,"soft_fail":false,"target_va":5379908992,"va":5407357510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_straight_halftimber_span
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8fa120000","size":5,"soft_fail":false,"target_va":5407361840,"va":5407356977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_riser
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8c4efffff","size":5,"soft_fail":false,"target_va":5407354336,"va":5407358487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringer_plank
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e88e050000","size":5,"soft_fail":false,"target_va":5407360448,"va":5407359021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::wooden_corners::emit_corner_band
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8c9160000","size":5,"soft_fail":false,"target_va":5407363552,"va":5407357714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::as_f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9284322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e81e5572ff","size":5,"soft_fail":false,"target_va":5398073024,"va":5407357341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24054577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407356528,"raw_hex":"e8cff490fe","size":5,"soft_fail":false,"target_va":5383304736,"va":5407359308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_risers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":4760,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":117738,"source_line":250,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":5152,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":117739,"source_line":368,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_treads
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":3672,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":117736,"source_line":188,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":3476,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":3476,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":3476,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4540,"parent_record_offset":4500,"ancestor_record_offset":3476,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4472,"parent_record_offset":4404,"ancestor_record_offset":3476,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":3476,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3140,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":3476,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3140,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":3476,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3140,"record_offset":7144,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":5376,"parent_record_offset":5344,"ancestor_record_offset":3476,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":4696,"parent_record_offset":4404,"ancestor_record_offset":3476,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":3476,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3140,"record_offset":3896,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":5092,"parent_record_offset":4760,"ancestor_record_offset":3476,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":4376,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":3476,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":3476,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":3476,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":3476,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3140,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":3476,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3140,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":3476,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3140,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":3476,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3140,"record_offset":4032,"parent_record_offset":3968,"ancestor_record_offset":3476,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":4660,"parent_record_offset":4500,"ancestor_record_offset":3476,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3140,"record_offset":3812,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3140,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3476,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3140,"record_offset":3968,"parent_record_offset":3896,"ancestor_record_offset":3476,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":3476,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6684,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":4112,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":4200,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6628,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6804,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":7660,"parent_record_offset":7600,"ancestor_record_offset":3476,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":5036,"parent_record_offset":4760,"ancestor_record_offset":3476,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":4980,"parent_record_offset":4760,"ancestor_record_offset":3476,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3140,"record_offset":4872,"parent_record_offset":4840,"ancestor_record_offset":3476,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3140,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":3476,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":6740,"parent_record_offset":6708,"ancestor_record_offset":3476,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":7600,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":117742,"source_line":404,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":6440,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":117741,"source_line":433,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_tread
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":4404,"parent_record_offset":3672,"ancestor_record_offset":3476,"inlinee":117737,"source_line":495,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3140,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":3476,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3140,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":3476,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":3476,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":6008,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":79567,"source_line":65,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5344,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5468,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":27119,"source_line":48,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5404,"parent_record_offset":5344,"ancestor_record_offset":3476,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":6044,"parent_record_offset":5152,"ancestor_record_offset":3476,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":7180,"parent_record_offset":7144,"ancestor_record_offset":3476,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::wooden_stair_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":1164,"rva":38646160,"va":5407355280}
// 编译实例 RVAs: 0x24db190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31677008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e8b0a51cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407355691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31677729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e8dfa21cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407356428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38633486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e8f27fb2fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407355673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8994700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e88c3f8900","size":5,"soft_fail":false,"target_va":5416350832,"va":5407356127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9702179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e8230b9400","size":5,"soft_fail":false,"target_va":5417057664,"va":5407355480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9265130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407355280,"raw_hex":"e8ea5f8d00","size":5,"soft_fail":false,"target_va":5416620794,"va":5407355659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":2960,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":1692,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":1580,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3140,"record_offset":2112,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":3180,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":1460,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":2244,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":1384,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":1608,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::plan_offset_at_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":2508,"parent_record_offset":1164,"ancestor_record_offset":1164,"inlinee":117735,"source_line":175,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1164,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":1164,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":1164,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":1164,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1164,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":1164,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":1980,"parent_record_offset":1796,"ancestor_record_offset":1164,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3140,"record_offset":3344,"parent_record_offset":3252,"ancestor_record_offset":1164,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":1164,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1164,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":1164,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":1164,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":1164,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":1164,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":1164,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1164,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":1164,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":1836,"parent_record_offset":1796,"ancestor_record_offset":1164,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":3032,"parent_record_offset":2992,"ancestor_record_offset":1164,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":2992,"parent_record_offset":2960,"ancestor_record_offset":1164,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1164,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3140,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1164,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3140,"record_offset":2176,"parent_record_offset":2140,"ancestor_record_offset":1164,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":2844,"parent_record_offset":2508,"ancestor_record_offset":1164,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":1164,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1164,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3140,"record_offset":1940,"parent_record_offset":1884,"ancestor_record_offset":1164,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3140,"record_offset":2140,"parent_record_offset":2112,"ancestor_record_offset":1164,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1164,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":1164,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":3312,"parent_record_offset":3280,"ancestor_record_offset":1164,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":2444,"parent_record_offset":2416,"ancestor_record_offset":1164,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":1164,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":1164,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3140,"record_offset":2208,"parent_record_offset":2140,"ancestor_record_offset":1164,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":1164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":3144,"parent_record_offset":2992,"ancestor_record_offset":1164,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2552,"parent_record_offset":2508,"ancestor_record_offset":1164,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2632,"parent_record_offset":2576,"ancestor_record_offset":1164,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":1164,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1164,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1164,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":2416,"parent_record_offset":2244,"ancestor_record_offset":1164,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":1516,"parent_record_offset":1484,"ancestor_record_offset":1164,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::plan_offset_at_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBE7
// 首条位置: {"module":3140,"inlinee":117735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":2844,"parent_record_offset":2508,"ancestor_record_offset":2508,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2552,"parent_record_offset":2508,"ancestor_record_offset":2508,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2788,"parent_record_offset":2508,"ancestor_record_offset":2508,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2732,"parent_record_offset":2508,"ancestor_record_offset":2508,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":2508,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":2604,"parent_record_offset":2576,"ancestor_record_offset":2508,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2632,"parent_record_offset":2576,"ancestor_record_offset":2508,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2508,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":2664,"parent_record_offset":2632,"ancestor_record_offset":2508,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_treads
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBE8
// 首条位置: {"module":3140,"inlinee":117736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3140,"record_offset":3896,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":4376,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3140,"record_offset":3812,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":4112,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":4200,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_tread
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":4404,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":117737,"source_line":495,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":3672,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":3672,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":3672,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4540,"parent_record_offset":4500,"ancestor_record_offset":3672,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4472,"parent_record_offset":4404,"ancestor_record_offset":3672,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":3672,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":4696,"parent_record_offset":4404,"ancestor_record_offset":3672,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":3672,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":3672,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3140,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":3672,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3140,"record_offset":4032,"parent_record_offset":3968,"ancestor_record_offset":3672,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":4660,"parent_record_offset":4500,"ancestor_record_offset":3672,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3140,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3672,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3140,"record_offset":3968,"parent_record_offset":3896,"ancestor_record_offset":3672,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_risers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBEA
// 首条位置: {"module":3140,"inlinee":117738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3140,"record_offset":4840,"parent_record_offset":4760,"ancestor_record_offset":4760,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":5092,"parent_record_offset":4760,"ancestor_record_offset":4760,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3140,"record_offset":5120,"parent_record_offset":4760,"ancestor_record_offset":4760,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":5036,"parent_record_offset":4760,"ancestor_record_offset":4760,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":4980,"parent_record_offset":4760,"ancestor_record_offset":4760,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":4760,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4760,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3140,"record_offset":4936,"parent_record_offset":4904,"ancestor_record_offset":4760,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3140,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":4760,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3140,"record_offset":4872,"parent_record_offset":4840,"ancestor_record_offset":4760,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_riser
// 记录形式: compiled_procedure; instances=1
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":280,"rva":38645216,"va":5407354336}
// 编译实例 RVAs: 0x24dade0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24029698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e8fe5591fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407354749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9025470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e8beb78900","size":5,"soft_fail":false,"target_va":5416380336,"va":5407354861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27446411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e875335dfe","size":5,"soft_fail":false,"target_va":5379908400,"va":5407354806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27446993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e82f315dfe","size":5,"soft_fail":false,"target_va":5379907856,"va":5407354844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27445841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e8af355dfe","size":5,"soft_fail":false,"target_va":5379908992,"va":5407354828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_straight_halftimber_span
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e80a1a0000","size":5,"soft_fail":false,"target_va":5407361840,"va":5407355169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::wooden_corners::emit_corner_band
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e839210000","size":5,"soft_fail":false,"target_va":5407363552,"va":5407355042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::as_f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9281603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407354336,"raw_hex":"e8bd5f72ff","size":5,"soft_fail":false,"target_va":5398073024,"va":5407354622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":680,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":596,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":512,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":3140,"record_offset":540,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":896,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":984,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":740,"parent_record_offset":704,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":860,"parent_record_offset":704,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBEB
// 首条位置: {"module":3140,"inlinee":117739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3140,"record_offset":7144,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6684,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6376,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6916,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6628,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6804,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":7600,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":117742,"source_line":404,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":3140,"record_offset":6440,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":117741,"source_line":433,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5588,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":6008,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":79567,"source_line":65,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5344,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5468,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":27119,"source_line":48,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":6044,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3140,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":5152,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3140,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":5152,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":5376,"parent_record_offset":5344,"ancestor_record_offset":5152,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":5152,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6224,"parent_record_offset":6192,"ancestor_record_offset":5152,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5152,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":5152,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":5152,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3140,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":5152,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3140,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":5152,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":5152,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6488,"parent_record_offset":6440,"ancestor_record_offset":5152,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6560,"parent_record_offset":6440,"ancestor_record_offset":5152,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":7660,"parent_record_offset":7600,"ancestor_record_offset":5152,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":7780,"parent_record_offset":7600,"ancestor_record_offset":5152,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3140,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":5152,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":6740,"parent_record_offset":6708,"ancestor_record_offset":5152,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3140,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":5152,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3140,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":5152,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":6284,"parent_record_offset":6044,"ancestor_record_offset":5152,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":5404,"parent_record_offset":5344,"ancestor_record_offset":5152,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3140,"record_offset":7180,"parent_record_offset":7144,"ancestor_record_offset":5152,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBEE
// 首条位置: {"module":3140,"inlinee":117742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":7660,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":7780,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":7824,"parent_record_offset":7780,"ancestor_record_offset":7600,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":7704,"parent_record_offset":7660,"ancestor_record_offset":7600,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 433
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBED
// 首条位置: {"module":3140,"inlinee":117741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6488,"parent_record_offset":6440,"ancestor_record_offset":6440,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":6560,"parent_record_offset":6440,"ancestor_record_offset":6440,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":6440,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":6520,"parent_record_offset":6488,"ancestor_record_offset":6440,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringer_plank
// 记录形式: compiled_procedure; instances=1
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":9352,"rva":38651328,"va":5407360448}
// 编译实例 RVAs: 0x24dc5c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37819596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360448,"raw_hex":"e834ebbefd","size":5,"soft_fail":false,"target_va":5369542000,"va":5407361591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29234662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360448,"raw_hex":"e81aea41fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5407361601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25635042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360448,"raw_hex":"e81ed778fe","size":5,"soft_fail":false,"target_va":5381726576,"va":5407361613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9018690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360448,"raw_hex":"e8429d8900","size":5,"soft_fail":false,"target_va":5416380336,"va":5407361641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-23953330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407360448,"raw_hex":"e84e8092fe","size":5,"soft_fail":false,"target_va":5383408256,"va":5407361581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":12700,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3140,"record_offset":12672,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_mat3_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":3140,"record_offset":12408,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":117743,"source_line":238,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":3140,"record_offset":9724,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9822,"source_line":843,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11920,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11220,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11044,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11648,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":9660,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11864,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":10844,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":9588,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11164,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":12800,"parent_record_offset":12768,"ancestor_record_offset":9352,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":12848,"parent_record_offset":12824,"ancestor_record_offset":9352,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":12824,"parent_record_offset":12800,"ancestor_record_offset":9352,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":12768,"parent_record_offset":12728,"ancestor_record_offset":9352,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":12728,"parent_record_offset":12700,"ancestor_record_offset":9352,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3140,"record_offset":12440,"parent_record_offset":12408,"ancestor_record_offset":9352,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":9352,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":9996,"parent_record_offset":9936,"ancestor_record_offset":9352,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":9352,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":9352,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":10880,"parent_record_offset":10844,"ancestor_record_offset":9352,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":9352,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":12888,"parent_record_offset":12728,"ancestor_record_offset":9352,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":3140,"record_offset":10808,"parent_record_offset":9724,"ancestor_record_offset":9352,"inlinee":48855,"source_line":94,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":3140,"record_offset":9828,"parent_record_offset":9792,"ancestor_record_offset":9352,"inlinee":9826,"source_line":625,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":3140,"record_offset":9792,"parent_record_offset":9724,"ancestor_record_offset":9352,"inlinee":9824,"source_line":893,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$50::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":3140,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":9352,"inlinee":117745,"source_line":1187,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3140,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":9352,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3140,"record_offset":12516,"parent_record_offset":12492,"ancestor_record_offset":9352,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11068,"parent_record_offset":11044,"ancestor_record_offset":9352,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":9352,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11700,"parent_record_offset":11672,"ancestor_record_offset":9352,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":11672,"parent_record_offset":11648,"ancestor_record_offset":9352,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":9872,"parent_record_offset":9828,"ancestor_record_offset":9352,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3140,"record_offset":10388,"parent_record_offset":10164,"ancestor_record_offset":9352,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":11100,"parent_record_offset":11068,"ancestor_record_offset":9352,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_tread
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 495
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBE9
// 首条位置: {"module":3140,"inlinee":117737}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4472,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":4696,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":4404,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4404,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4404,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4540,"parent_record_offset":4500,"ancestor_record_offset":4404,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4404,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":4660,"parent_record_offset":4500,"ancestor_record_offset":4404,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_straight_halftimber_span
// 记录形式: compiled_procedure; instances=1
// 源码行: 533
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3140,"record_offset":12928,"rva":38652720,"va":5407361840}
// 编译实例 RVAs: 0x24dcb30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: emit_wooden_stair_pieces.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24037042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407361840,"raw_hex":"e84e3991fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407362093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9018109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407361840,"raw_hex":"e8fd9a8900","size":5,"soft_fail":false,"target_va":5416380336,"va":5407362222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight_with_depth
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27453324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407361840,"raw_hex":"e874185dfe","size":5,"soft_fail":false,"target_va":5379908848,"va":5407362167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27454354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407361840,"raw_hex":"e86e145dfe","size":5,"soft_fail":false,"target_va":5379907856,"va":5407362205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27453202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407361840,"raw_hex":"e8ee185dfe","size":5,"soft_fail":false,"target_va":5379908992,"va":5407362189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":13892,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13712,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13200,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13376,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13544,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13144,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13320,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":13984,"parent_record_offset":13952,"ancestor_record_offset":12928,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":12928,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3140,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":12928,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":13952,"parent_record_offset":13916,"ancestor_record_offset":12928,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3140,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":12928,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3140,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":12928,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12928,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":12928,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":12928,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3140,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":12928,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3140,"record_offset":14072,"parent_record_offset":13916,"ancestor_record_offset":12928,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":12928,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3140,"record_offset":13736,"parent_record_offset":13712,"ancestor_record_offset":12928,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03140.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3140,"record_offset":13256,"parent_record_offset":13224,"ancestor_record_offset":12928,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03140.jsonl.gz"}
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
// 完整结构及原始类型记录: emit_wooden_stair_pieces.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,alloc::alloc::Global>", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["country_core::resources::walls::WallId", "system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::IntoIter<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::IntoIter<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPieces,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPoint
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPoint,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::WoodenStairPoint", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: emit_wooden_stair_pieces.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: emit_wooden_stair_pieces.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_straight_halftimber_span
// 信息 ID: 1745384c0532be28fc8439c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_riser @ 0x24db121 -> 0x24dcb30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24db831 -> 0x24dcb30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_riser
// 信息 ID: 622b68c6b396e7215ba8f59f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dbe17 -> 0x24dade0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dbe74 -> 0x24dade0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dbee2 -> 0x24dade0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_risers
// 信息 ID: 5f9aa5b7160f8860a5fab8d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=4760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringer_plank
// 信息 ID: b8fbf7c192d04ef7d9847612
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc272 -> 0x24dc5c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc3e5 -> 0x24dc5c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc02d -> 0x24dc5c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc0d3 -> 0x24dc5c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers
// 信息 ID: 52a0470eed603d662e4a227b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=5152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$0
// 信息 ID: a4ddf3271c7e2dc3c3b7ba83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=7600; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=7896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers::closure$1
// 信息 ID: cd73b575ca2f049e4b8eea64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_stringers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=6440; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=6972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_tread
// 信息 ID: 90ebde3aa1862fbab6998b0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=4404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_treads
// 信息 ID: 445ebe779b9412b24677ac77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout; module=3140, inline_record=3672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout
// 信息 ID: 0c9bf670b451f928c5d3ecd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141cb72 -> 0x24db670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c049 -> 0x24db670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_pieces @ 0x24dc54b -> 0x24db670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24ddf1b -> 0x24db670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_pieces
// 信息 ID: 2b0fe5815e5f612ef61d71e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1433f8d -> 0x24dc4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$3::default
// 信息 ID: 99106854371ac032f5492bf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=395, inline_record=14572; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=12432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$4::clone
// 信息 ID: 03c675ed2e6e4d2f2b2f3a66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=8752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::plan_offset_at_point
// 信息 ID: e9dca98ab2d924e4c42c0333
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::wooden_stair_layout; module=3140, inline_record=2508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::wooden_stair_layout
// 信息 ID: 669042c226bec0e1e83cbef4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141ca42 -> 0x24db190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141bf14 -> 0x24db190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_pieces @ 0x24dc4fe -> 0x24db190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
