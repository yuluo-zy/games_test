// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8e665331f4bcc8edbcef05e32031f1a0260651a12537d266c9d9ed6698ea8d02
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_assemble_wood_misc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 432, 464, 466, 1545, 1600
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.24.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":464,"record_offset":8480,"rva":9089632,"va":5377798752}
// 编译实例 RVAs: 0x8ab260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2120229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e8dba5dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377798912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2122229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e80b9edfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377800928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9076706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e81e8075ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377798893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5525522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e812505400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377799497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38581895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e887b64c02","size":5,"soft_fail":false,"target_va":5416380896,"va":5377798996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38821916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e81c605002","size":5,"soft_fail":false,"target_va":5416620794,"va":5377798873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38821000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377798752,"raw_hex":"e8885c5002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377800757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":9132,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":8716,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":464,"record_offset":11048,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":9372,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":10464,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":11572,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":10232,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":9516,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10260,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10016,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":9728,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10932,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":8480,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":8480,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":8480,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":8480,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":8480,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":8480,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":9000,"parent_record_offset":8820,"ancestor_record_offset":8480,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":11736,"parent_record_offset":11644,"ancestor_record_offset":8480,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":9248,"parent_record_offset":9216,"ancestor_record_offset":8480,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":8480,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":11672,"parent_record_offset":11644,"ancestor_record_offset":8480,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":8480,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":8480,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":8480,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":8820,"parent_record_offset":8788,"ancestor_record_offset":8480,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":8480,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":8480,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":8856,"parent_record_offset":8820,"ancestor_record_offset":8480,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":9216,"parent_record_offset":9168,"ancestor_record_offset":8480,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":9168,"parent_record_offset":9132,"ancestor_record_offset":8480,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":8480,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":464,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":8480,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":8480,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":11188,"parent_record_offset":11156,"ancestor_record_offset":8480,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":8480,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":8960,"parent_record_offset":8904,"ancestor_record_offset":8480,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":464,"record_offset":10520,"parent_record_offset":10492,"ancestor_record_offset":8480,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":464,"record_offset":10640,"parent_record_offset":10548,"ancestor_record_offset":8480,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":9452,"parent_record_offset":9396,"ancestor_record_offset":8480,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":8480,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":8480,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8480,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":8480,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":11704,"parent_record_offset":11672,"ancestor_record_offset":8480,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":10800,"parent_record_offset":10752,"ancestor_record_offset":8480,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":8480,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":9572,"parent_record_offset":9540,"ancestor_record_offset":8480,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":464,"record_offset":10712,"parent_record_offset":10640,"ancestor_record_offset":8480,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":11596,"parent_record_offset":11572,"ancestor_record_offset":8480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":10612,"parent_record_offset":10548,"ancestor_record_offset":8480,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":9336,"parent_record_offset":9168,"ancestor_record_offset":8480,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":10548,"parent_record_offset":10520,"ancestor_record_offset":8480,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":11256,"parent_record_offset":11228,"ancestor_record_offset":8480,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":9628,"parent_record_offset":9540,"ancestor_record_offset":8480,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":8480,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10068,"parent_record_offset":10040,"ancestor_record_offset":8480,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10040,"parent_record_offset":10016,"ancestor_record_offset":8480,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10752,"parent_record_offset":10712,"ancestor_record_offset":8480,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":8480,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$84::sum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":10492,"parent_record_offset":10464,"ancestor_record_offset":8480,"inlinee":26364,"source_line":2022,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":9660,"parent_record_offset":9628,"ancestor_record_offset":8480,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":10668,"parent_record_offset":10640,"ancestor_record_offset":8480,"inlinee":27502,"source_line":446,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":11228,"parent_record_offset":11128,"ancestor_record_offset":8480,"inlinee":27503,"source_line":449,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::construct_wooden_stair_steps_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":464,"record_offset":34860,"rva":9100896,"va":5377810016}
// 编译实例 RVAs: 0x8ade60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ab390
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7352039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e819d18fff","size":5,"soft_fail":false,"target_va":5370459024,"va":5377811058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t316267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e86bd30400","size":5,"soft_fail":false,"target_va":5378126944,"va":5377810672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3915889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e871c03b00","size":5,"soft_fail":false,"target_va":5381726576,"va":5377810682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5460919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8b7535300","size":5,"soft_fail":false,"target_va":5383273968,"va":5377813044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5513408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8c0205400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377811611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5513693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8dd215400","size":5,"soft_fail":false,"target_va":5383325056,"va":5377811358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b6a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12617932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8cc88c000","size":5,"soft_fail":false,"target_va":5390428736,"va":5377810799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414f85a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12887526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8e6a5c400","size":5,"soft_fail":false,"target_va":5390697888,"va":5377810357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39244426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e88ad25602","size":5,"soft_fail":false,"target_va":5417057696,"va":5377813265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39244774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8e6d35602","size":5,"soft_fail":false,"target_va":5417058032,"va":5377813253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5572037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e8c5055500","size":5,"soft_fail":false,"target_va":5383383152,"va":5377811110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12892962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377810016,"raw_hex":"e822bbc400","size":5,"soft_fail":false,"target_va":5390703600,"va":5377810633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":464,"record_offset":36548,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":41328,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":36576,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":41104,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":464,"record_offset":36048,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":35292,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":41132,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":41188,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":464,"record_offset":35180,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":464,"record_offset":36840,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":8966,"source_line":122,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":464,"record_offset":41784,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":26854,"source_line":146,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":464,"record_offset":42152,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":36992,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37244,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37216,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40320,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":39732,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":39404,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40668,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":36604,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":39980,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":36136,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":37940,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":36228,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":37672,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40196,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":39524,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":464,"record_offset":37076,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::shader::math::Affine3Packed::from_cols
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":464,"record_offset":36404,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":27384,"source_line":42,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/lib.rs
//     evidence={"module":464,"record_offset":36876,"parent_record_offset":36840,"ancestor_record_offset":34860,"inlinee":8834,"source_line":408,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":464,"record_offset":36900,"parent_record_offset":36876,"ancestor_record_offset":34860,"inlinee":8836,"source_line":254,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":464,"record_offset":36924,"parent_record_offset":36900,"ancestor_record_offset":34860,"inlinee":8837,"source_line":353,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":35500,"parent_record_offset":35476,"ancestor_record_offset":34860,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":35476,"parent_record_offset":35452,"ancestor_record_offset":34860,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":464,"record_offset":42208,"parent_record_offset":42180,"ancestor_record_offset":34860,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":464,"record_offset":42716,"parent_record_offset":42292,"ancestor_record_offset":34860,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":464,"record_offset":42420,"parent_record_offset":42364,"ancestor_record_offset":34860,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":464,"record_offset":42392,"parent_record_offset":42364,"ancestor_record_offset":34860,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":464,"record_offset":42596,"parent_record_offset":42512,"ancestor_record_offset":34860,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":464,"record_offset":42660,"parent_record_offset":42292,"ancestor_record_offset":34860,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":37388,"parent_record_offset":37364,"ancestor_record_offset":34860,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":41156,"parent_record_offset":41132,"ancestor_record_offset":34860,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":35428,"parent_record_offset":35292,"ancestor_record_offset":34860,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":35452,"parent_record_offset":35428,"ancestor_record_offset":34860,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":36776,"parent_record_offset":36744,"ancestor_record_offset":34860,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=59
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":36164,"parent_record_offset":36136,"ancestor_record_offset":34860,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":39552,"parent_record_offset":39524,"ancestor_record_offset":34860,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":40228,"parent_record_offset":40196,"ancestor_record_offset":34860,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":36256,"parent_record_offset":36228,"ancestor_record_offset":34860,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":37040,"parent_record_offset":37016,"ancestor_record_offset":34860,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":464,"record_offset":36092,"parent_record_offset":36048,"ancestor_record_offset":34860,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":464,"record_offset":35252,"parent_record_offset":35228,"ancestor_record_offset":34860,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":464,"record_offset":35204,"parent_record_offset":35180,"ancestor_record_offset":34860,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":464,"record_offset":35228,"parent_record_offset":35204,"ancestor_record_offset":34860,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":464,"record_offset":35316,"parent_record_offset":35292,"ancestor_record_offset":34860,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":464,"record_offset":41816,"parent_record_offset":41784,"ancestor_record_offset":34860,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":464,"record_offset":41904,"parent_record_offset":41784,"ancestor_record_offset":34860,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":464,"record_offset":42180,"parent_record_offset":42152,"ancestor_record_offset":34860,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":464,"record_offset":42292,"parent_record_offset":42180,"ancestor_record_offset":34860,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":464,"record_offset":42744,"parent_record_offset":42292,"ancestor_record_offset":34860,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":464,"record_offset":42256,"parent_record_offset":42232,"ancestor_record_offset":34860,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":464,"record_offset":42232,"parent_record_offset":42208,"ancestor_record_offset":34860,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37332,"parent_record_offset":37300,"ancestor_record_offset":34860,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37300,"parent_record_offset":37268,"ancestor_record_offset":34860,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37268,"parent_record_offset":37244,"ancestor_record_offset":34860,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":34860,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":39428,"parent_record_offset":39404,"ancestor_record_offset":34860,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":34860,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40032,"parent_record_offset":40004,"ancestor_record_offset":34860,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":40004,"parent_record_offset":39980,"ancestor_record_offset":34860,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":36628,"parent_record_offset":36604,"ancestor_record_offset":34860,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":36744,"parent_record_offset":36604,"ancestor_record_offset":34860,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":464,"record_offset":42364,"parent_record_offset":42340,"ancestor_record_offset":34860,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":464,"record_offset":42340,"parent_record_offset":42292,"ancestor_record_offset":34860,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":41864,"parent_record_offset":41840,"ancestor_record_offset":34860,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":41840,"parent_record_offset":41816,"ancestor_record_offset":34860,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":37364,"parent_record_offset":37332,"ancestor_record_offset":34860,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers
// 记录形式: compiled_procedure; instances=1
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":464,"record_offset":272,"rva":9085472,"va":5377794592}
// 编译实例 RVAs: 0x8aa220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2116961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e89fb2dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377795644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2118472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e8b8acdfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377797171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9073442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e8de8c75ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377795629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408abb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e8c00d0000","size":5,"soft_fail":false,"target_va":5377801024,"va":5377797499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a39b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12555277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e80d94bf00","size":5,"soft_fail":false,"target_va":5390350768,"va":5377795486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12560773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e885a9bf00","size":5,"soft_fail":false,"target_va":5390358128,"va":5377797350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a8ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12573836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e88cdcbf00","size":5,"soft_fail":false,"target_va":5390372576,"va":5377798735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38553221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e885464c02","size":5,"soft_fail":false,"target_va":5416350496,"va":5377797270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38555943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e827514c02","size":5,"soft_fail":false,"target_va":5416352064,"va":5377796116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38823357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377794592,"raw_hex":"e8bd655002","size":5,"soft_fail":false,"target_va":5416620794,"va":5377797432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1016,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1044,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":1604,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::floor_offsets
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":464,"record_offset":480,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27386,"source_line":20,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stringer_side_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":1756,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27497,"source_line":257,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2828,"parent_record_offset":2632,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":1464,"parent_record_offset":1376,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1164,"parent_record_offset":1132,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2560,"parent_record_offset":2520,"ancestor_record_offset":272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2596,"parent_record_offset":2560,"ancestor_record_offset":272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2668,"parent_record_offset":2632,"ancestor_record_offset":272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2960,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":272,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":3032,"parent_record_offset":2996,"ancestor_record_offset":272,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1132,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3396,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1080,"parent_record_offset":1044,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2480,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2520,"parent_record_offset":2480,"ancestor_record_offset":272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2060,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":464,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":464,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":464,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":272,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":464,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":464,"record_offset":584,"parent_record_offset":528,"ancestor_record_offset":272,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":464,"record_offset":624,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":464,"record_offset":7552,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":6131,"source_line":36,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":4780,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":1664,"parent_record_offset":1632,"ancestor_record_offset":272,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":272,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":1692,"parent_record_offset":1632,"ancestor_record_offset":272,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":1632,"parent_record_offset":1604,"ancestor_record_offset":272,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":3616,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":464,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":1972,"parent_record_offset":1856,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":272,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":464,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":272,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2788,"parent_record_offset":2724,"ancestor_record_offset":272,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":4752,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":272,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":5252,"parent_record_offset":5220,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":464,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":272,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":464,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":3328,"parent_record_offset":3032,"ancestor_record_offset":272,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":1280,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":1252,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":272,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":272,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4156,"parent_record_offset":4124,"ancestor_record_offset":272,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":2144,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":272,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4096,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4860,"parent_record_offset":4808,"ancestor_record_offset":272,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5220,"parent_record_offset":4808,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":3920,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":4244,"parent_record_offset":4212,"ancestor_record_offset":272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":464,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":272,"inlinee":27388,"source_line":21,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":464,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":272,"inlinee":27390,"source_line":23,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::catmull_rom_vec3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":4808,"parent_record_offset":2324,"ancestor_record_offset":272,"inlinee":27499,"source_line":686,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":6828,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":27500,"source_line":672,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::densify_railing_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":2324,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":27498,"source_line":630,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":1856,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":26738,"source_line":340,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stringer_side_points
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B69
// 首条位置: {"module":464,"inlinee":27497}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2060,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":464,"record_offset":7552,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":6131,"source_line":36,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":7876,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":2144,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":6828,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":27500,"source_line":672,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::densify_railing_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":2324,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":27498,"source_line":630,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":1856,"parent_record_offset":1756,"ancestor_record_offset":1756,"inlinee":26738,"source_line":340,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":1756,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":1756,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":1756,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":1756,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":1756,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":1756,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2828,"parent_record_offset":2632,"ancestor_record_offset":1756,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6420,"parent_record_offset":6328,"ancestor_record_offset":1756,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":1756,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2560,"parent_record_offset":2520,"ancestor_record_offset":1756,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":1756,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":1756,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":1756,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":1756,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":1756,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2596,"parent_record_offset":2560,"ancestor_record_offset":1756,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":1756,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2668,"parent_record_offset":2632,"ancestor_record_offset":1756,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2960,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":1756,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":3032,"parent_record_offset":2996,"ancestor_record_offset":1756,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":1756,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3396,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3424,"parent_record_offset":3396,"ancestor_record_offset":1756,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2480,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2520,"parent_record_offset":2480,"ancestor_record_offset":1756,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":1756,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":464,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":1756,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":4780,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":1756,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":1756,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":1756,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1756,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":464,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1756,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":1756,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":3616,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":464,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1756,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":1972,"parent_record_offset":1856,"ancestor_record_offset":1756,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":1756,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":1756,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2788,"parent_record_offset":2724,"ancestor_record_offset":1756,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":4752,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":6388,"parent_record_offset":6356,"ancestor_record_offset":1756,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":5252,"parent_record_offset":5220,"ancestor_record_offset":1756,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":1756,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":1756,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":3328,"parent_record_offset":3032,"ancestor_record_offset":1756,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":1756,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":6256,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":3580,"parent_record_offset":3424,"ancestor_record_offset":1756,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1756,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1756,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":1756,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":1756,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4156,"parent_record_offset":4124,"ancestor_record_offset":1756,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":1756,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4096,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1756,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4860,"parent_record_offset":4808,"ancestor_record_offset":1756,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5220,"parent_record_offset":4808,"ancestor_record_offset":1756,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":3920,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":4244,"parent_record_offset":4212,"ancestor_record_offset":1756,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::catmull_rom_vec3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":4808,"parent_record_offset":2324,"ancestor_record_offset":1756,"inlinee":27499,"source_line":686,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece::railing
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B74
// 首条位置: {"module":1545,"inlinee":27508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B78
// 首条位置: {"module":466,"inlinee":27512}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":20524,"parent_record_offset":20492,"ancestor_record_offset":20492,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":466,"record_offset":20580,"parent_record_offset":20492,"ancestor_record_offset":20492,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":466,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":20492,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":466,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":20492,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":466,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":20492,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00466.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 334
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13989
// 首条位置: {"module":1545,"inlinee":80265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1398A
// 首条位置: {"module":1545,"inlinee":80266}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece::railing
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":1545,"record_offset":10300,"parent_record_offset":9552,"ancestor_record_offset":9552,"inlinee":27508,"source_line":314,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":1545,"record_offset":9580,"parent_record_offset":9552,"ancestor_record_offset":9552,"inlinee":27500,"source_line":672,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":9552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":9552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":9948,"parent_record_offset":9924,"ancestor_record_offset":9552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":9876,"parent_record_offset":9784,"ancestor_record_offset":9552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1545,"record_offset":9812,"parent_record_offset":9784,"ancestor_record_offset":9552,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1545,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":9552,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1545,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":9552,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1545,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9552,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1545,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":9552,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1545,"record_offset":9628,"parent_record_offset":9580,"ancestor_record_offset":9552,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1545,"record_offset":9844,"parent_record_offset":9812,"ancestor_record_offset":9552,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":9712,"parent_record_offset":9580,"ancestor_record_offset":9552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6872
// 首条位置: {"module":432,"inlinee":26738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":12256,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":12280,"parent_record_offset":12256,"ancestor_record_offset":12140,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":12140,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":12140,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":12140,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13B41
// 首条位置: {"module":1600,"inlinee":80705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":6648,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1600,"record_offset":6592,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":6448,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":6504,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":6396,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6396,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":6396,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces
// 记录形式: compiled_procedure; instances=1
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":464,"record_offset":11868,"rva":9091904,"va":5377801024}
// 编译实例 RVAs: 0x8abb40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2122938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377801024,"raw_hex":"e8469bdfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377801637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12571485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377801024,"raw_hex":"e85dd3bf00","size":5,"soft_fail":false,"target_va":5390373440,"va":5377801950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38551240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377801024,"raw_hex":"e8c83e4c02","size":5,"soft_fail":false,"target_va":5416352736,"va":5377801491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377801024,"raw_hex":"e828f6ffff","size":5,"soft_fail":false,"target_va":5377798752,"va":5377801267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-2123098","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377801024,"raw_hex":"e9a69adfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377801797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":12812,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":12080,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":12196,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":12344,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":13388,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":13268,"parent_record_offset":13244,"ancestor_record_offset":11868,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":11868,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":11868,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":13244,"parent_record_offset":13156,"ancestor_record_offset":11868,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":12920,"parent_record_offset":12888,"ancestor_record_offset":11868,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":13184,"parent_record_offset":13156,"ancestor_record_offset":11868,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":11868,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":11868,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":11868,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":13132,"parent_record_offset":13108,"ancestor_record_offset":11868,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":12888,"parent_record_offset":12844,"ancestor_record_offset":11868,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":12844,"parent_record_offset":12812,"ancestor_record_offset":11868,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":11868,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":14152,"parent_record_offset":14128,"ancestor_record_offset":11868,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":11868,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":464,"record_offset":12108,"parent_record_offset":12080,"ancestor_record_offset":11868,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":464,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":11868,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":13212,"parent_record_offset":13184,"ancestor_record_offset":11868,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":11868,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":13036,"parent_record_offset":12844,"ancestor_record_offset":11868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":464,"record_offset":12164,"parent_record_offset":12080,"ancestor_record_offset":11868,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":12256,"parent_record_offset":12196,"ancestor_record_offset":11868,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":12284,"parent_record_offset":12196,"ancestor_record_offset":11868,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":12228,"parent_record_offset":12196,"ancestor_record_offset":11868,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11868,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":13008,"parent_record_offset":12844,"ancestor_record_offset":11868,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":464,"record_offset":12380,"parent_record_offset":12344,"ancestor_record_offset":11868,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":12572,"parent_record_offset":12540,"ancestor_record_offset":11868,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":12540,"parent_record_offset":12416,"ancestor_record_offset":11868,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12704,"parent_record_offset":12676,"ancestor_record_offset":11868,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12676,"parent_record_offset":12416,"ancestor_record_offset":11868,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12452,"parent_record_offset":12416,"ancestor_record_offset":11868,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":12612,"parent_record_offset":12572,"ancestor_record_offset":11868,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":12416,"parent_record_offset":12344,"ancestor_record_offset":11868,"inlinee":27505,"source_line":380,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B71
// 首条位置: {"module":464,"inlinee":27505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":12540,"parent_record_offset":12416,"ancestor_record_offset":12416,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12676,"parent_record_offset":12416,"ancestor_record_offset":12416,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12452,"parent_record_offset":12416,"ancestor_record_offset":12416,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":12416,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":12416,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":464,"record_offset":12572,"parent_record_offset":12540,"ancestor_record_offset":12416,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":12704,"parent_record_offset":12676,"ancestor_record_offset":12416,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":12612,"parent_record_offset":12572,"ancestor_record_offset":12416,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 446
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6E
// 首条位置: {"module":464,"inlinee":27502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6F
// 首条位置: {"module":464,"inlinee":27503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":11256,"parent_record_offset":11228,"ancestor_record_offset":11228,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":11480,"parent_record_offset":11452,"ancestor_record_offset":11228,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":11340,"parent_record_offset":11288,"ancestor_record_offset":11228,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":11404,"parent_record_offset":11288,"ancestor_record_offset":11228,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":11288,"parent_record_offset":11256,"ancestor_record_offset":11228,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":11452,"parent_record_offset":11404,"ancestor_record_offset":11228,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::upload_wooden_frames_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":464,"record_offset":14772,"rva":9093024,"va":5377802144}
// 编译实例 RVAs: 0x8abfa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::hot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5660081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8b15d5600","size":5,"soft_fail":false,"target_va":5383464400,"va":5377804314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2124067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8dd96dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377802750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t-2124760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e82894dfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377803459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9080544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8207175ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377802731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8447647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e861197fff","size":5,"soft_fail":false,"target_va":5369358064,"va":5377805706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ccc0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4324861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e80302beff","size":5,"soft_fail":false,"target_va":5373480128,"va":5377804984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t307038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e85eaf0400","size":5,"soft_fail":false,"target_va":5378112288,"va":5377805245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t321706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8aae80400","size":5,"soft_fail":false,"target_va":5378126944,"va":5377805233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1912147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8532d1d00","size":5,"soft_fail":false,"target_va":5379716240,"va":5377804088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90f10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4078898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8323d3e00","size":5,"soft_fail":false,"target_va":5381885712,"va":5377806809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7334093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8cde86f00","size":5,"soft_fail":false,"target_va":5385139392,"va":5377805294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a8b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12562334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e89eafbf00","size":5,"soft_fail":false,"target_va":5390371632,"va":5377809293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a8bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12563864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e898b5bf00","size":5,"soft_fail":false,"target_va":5390371824,"va":5377807955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12565118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e87ebabf00","size":5,"soft_fail":false,"target_va":5390375056,"va":5377809933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12566452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8b4bfbf00","size":5,"soft_fail":false,"target_va":5390375184,"va":5377808727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9be0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12567810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e802c5bf00","size":5,"soft_fail":false,"target_va":5390375904,"va":5377808089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c75d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12690551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e877a4c100","size":5,"soft_fail":false,"target_va":5390497232,"va":5377806676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c7880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12692372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e894abc100","size":5,"soft_fail":false,"target_va":5390497920,"va":5377805543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414ed7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12849166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e80e10c400","size":5,"soft_fail":false,"target_va":5390653376,"va":5377804205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38546837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8952d4c02","size":5,"soft_fail":false,"target_va":5416350608,"va":5377803766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38547878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8a6314c02","size":5,"soft_fail":false,"target_va":5416351168,"va":5377803285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38549704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8c8384c02","size":5,"soft_fail":false,"target_va":5416353040,"va":5377803331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_assemble_wood_misc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39253808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e830f75602","size":5,"soft_fail":false,"target_va":5417057056,"va":5377803243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38813350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8a63e5002","size":5,"soft_fail":false,"target_va":5416620769,"va":5377807414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38719136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8a0ce4e02","size":5,"soft_fail":false,"target_va":5416524720,"va":5377805579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38818089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e829515002","size":5,"soft_fail":false,"target_va":5416620794,"va":5377802700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38814025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e849415002","size":5,"soft_fail":false,"target_va":5416621408,"va":5377807378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::num::int_log10::panic_for_nonpositive_argument
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38814941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8dd445002","size":5,"soft_fail":false,"target_va":5416622288,"va":5377807342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38814265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e839425002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377807492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e86f2f0000","size":5,"soft_fail":false,"target_va":5377814400,"va":5377802252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38813841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e891405002","size":5,"soft_fail":false,"target_va":5416621376,"va":5377807530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6161186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e822035e00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377802601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6169693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e85d245e00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377802526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6157990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8a6f65d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377806917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6160590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8ce005e00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377802221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5683430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8e6b85600","size":5,"soft_fail":false,"target_va":5383487568,"va":5377804133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38910303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e85fb95102","size":5,"soft_fail":false,"target_va":5416717696,"va":5377807388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12559380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e814a4bf00","size":5,"soft_fail":false,"target_va":5390367248,"va":5377807863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38811883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377802144,"raw_hex":"e8eb385002","size":5,"soft_fail":false,"target_va":5416619360,"va":5377807472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rbx","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377802144,"raw_hex":"ffd3","size":2,"soft_fail":false,"va":5377804361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":464,"record_offset":22236,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":464,"record_offset":24128,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::append
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":19108,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":6181,"source_line":2867,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18024,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":22064,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":17264,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18344,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":17292,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":464,"record_offset":18928,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":22716,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":22504,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":22568,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":464,"record_offset":17968,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":464,"record_offset":24216,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":464,"record_offset":18316,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":19516,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":16456,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":15056,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":15444,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":464,"record_offset":22120,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":464,"record_offset":23160,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::wooden_frame_mesh_atlas_part_idx
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":22812,"parent_record_offset":14772,"ancestor_record_offset":14772,"inlinee":27506,"source_line":616,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":17572,"parent_record_offset":17548,"ancestor_record_offset":14772,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":14772,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":14772,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":18832,"parent_record_offset":18808,"ancestor_record_offset":14772,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":14772,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":14772,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":17548,"parent_record_offset":17396,"ancestor_record_offset":14772,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":18784,"parent_record_offset":18696,"ancestor_record_offset":14772,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":464,"record_offset":22316,"parent_record_offset":22272,"ancestor_record_offset":14772,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":464,"record_offset":22272,"parent_record_offset":22236,"ancestor_record_offset":14772,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18084,"parent_record_offset":18060,"ancestor_record_offset":14772,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":19212,"parent_record_offset":19184,"ancestor_record_offset":14772,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":17340,"parent_record_offset":17316,"ancestor_record_offset":14772,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":14772,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18724,"parent_record_offset":18696,"ancestor_record_offset":14772,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18696,"parent_record_offset":18672,"ancestor_record_offset":14772,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":19272,"parent_record_offset":19240,"ancestor_record_offset":14772,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":14772,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18108,"parent_record_offset":18084,"ancestor_record_offset":14772,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":19240,"parent_record_offset":19212,"ancestor_record_offset":14772,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":17396,"parent_record_offset":17364,"ancestor_record_offset":14772,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":17364,"parent_record_offset":17340,"ancestor_record_offset":14772,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":18672,"parent_record_offset":18648,"ancestor_record_offset":14772,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":17436,"parent_record_offset":17396,"ancestor_record_offset":14772,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":464,"record_offset":28776,"parent_record_offset":28740,"ancestor_record_offset":14772,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":464,"record_offset":28740,"parent_record_offset":28716,"ancestor_record_offset":14772,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":19136,"parent_record_offset":19108,"ancestor_record_offset":14772,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18428,"parent_record_offset":18380,"ancestor_record_offset":14772,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18060,"parent_record_offset":18024,"ancestor_record_offset":14772,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18172,"parent_record_offset":18024,"ancestor_record_offset":14772,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18380,"parent_record_offset":18344,"ancestor_record_offset":14772,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":19184,"parent_record_offset":19136,"ancestor_record_offset":14772,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":18200,"parent_record_offset":18024,"ancestor_record_offset":14772,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":17316,"parent_record_offset":17292,"ancestor_record_offset":14772,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":464,"record_offset":19044,"parent_record_offset":18952,"ancestor_record_offset":14772,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":464,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":14772,"inlinee":4794,"source_line":174,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":30252,"parent_record_offset":30228,"ancestor_record_offset":14772,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":25768,"parent_record_offset":25744,"ancestor_record_offset":14772,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":464,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":14772,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":464,"record_offset":22200,"parent_record_offset":22144,"ancestor_record_offset":14772,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":464,"record_offset":22172,"parent_record_offset":22144,"ancestor_record_offset":14772,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":464,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":14772,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":464,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":14772,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":14772,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16812,"parent_record_offset":16784,"ancestor_record_offset":14772,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16732,"parent_record_offset":16704,"ancestor_record_offset":14772,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":14772,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16872,"parent_record_offset":16756,"ancestor_record_offset":14772,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":464,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":14772,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":23072,"parent_record_offset":22812,"ancestor_record_offset":14772,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":23860,"parent_record_offset":23836,"ancestor_record_offset":14772,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":464,"record_offset":23428,"parent_record_offset":23404,"ancestor_record_offset":14772,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":464,"record_offset":23512,"parent_record_offset":23488,"ancestor_record_offset":14772,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":23956,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":14772,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":464,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":14772,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":464,"record_offset":19580,"parent_record_offset":19540,"ancestor_record_offset":14772,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":14772,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":18752,"parent_record_offset":18724,"ancestor_record_offset":14772,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":23008,"parent_record_offset":22976,"ancestor_record_offset":14772,"inlinee":26404,"source_line":1751,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22976,"parent_record_offset":22812,"ancestor_record_offset":14772,"inlinee":26403,"source_line":1639,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22912,"parent_record_offset":22812,"ancestor_record_offset":14772,"inlinee":26401,"source_line":3806,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":14772,"inlinee":26402,"source_line":3775,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22884,"parent_record_offset":22812,"ancestor_record_offset":14772,"inlinee":26400,"source_line":2300,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22856,"parent_record_offset":22812,"ancestor_record_offset":14772,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":464,"record_offset":23660,"parent_record_offset":23632,"ancestor_record_offset":14772,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":464,"record_offset":23036,"parent_record_offset":23008,"ancestor_record_offset":14772,"inlinee":14770,"source_line":1790,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":464,"record_offset":23632,"parent_record_offset":23604,"ancestor_record_offset":14772,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":464,"record_offset":16604,"parent_record_offset":16564,"ancestor_record_offset":14772,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":15392,"parent_record_offset":15364,"ancestor_record_offset":14772,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":17220,"parent_record_offset":16564,"ancestor_record_offset":14772,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":14772,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":23452,"parent_record_offset":23428,"ancestor_record_offset":14772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=74
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":16936,"parent_record_offset":16704,"ancestor_record_offset":14772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":464,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":14772,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":464,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":14772,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":464,"record_offset":23776,"parent_record_offset":23752,"ancestor_record_offset":14772,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":19016,"parent_record_offset":18952,"ancestor_record_offset":14772,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":28808,"parent_record_offset":28776,"ancestor_record_offset":14772,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":14772,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":19660,"parent_record_offset":19636,"ancestor_record_offset":14772,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":22656,"parent_record_offset":22568,"ancestor_record_offset":14772,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":15840,"parent_record_offset":15816,"ancestor_record_offset":14772,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":464,"record_offset":16048,"parent_record_offset":16020,"ancestor_record_offset":14772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":19068,"parent_record_offset":19044,"ancestor_record_offset":14772,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":18548,"parent_record_offset":18380,"ancestor_record_offset":14772,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":464,"record_offset":16148,"parent_record_offset":16120,"ancestor_record_offset":14772,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":464,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":14772,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":464,"record_offset":18228,"parent_record_offset":18024,"ancestor_record_offset":14772,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":22744,"parent_record_offset":22716,"ancestor_record_offset":14772,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":464,"record_offset":22772,"parent_record_offset":22744,"ancestor_record_offset":14772,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":464,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":14772,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":15792,"parent_record_offset":15768,"ancestor_record_offset":14772,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":15816,"parent_record_offset":15792,"ancestor_record_offset":14772,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":19636,"parent_record_offset":19540,"ancestor_record_offset":14772,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":16176,"parent_record_offset":16148,"ancestor_record_offset":14772,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":18952,"parent_record_offset":18928,"ancestor_record_offset":14772,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":464,"record_offset":15768,"parent_record_offset":15652,"ancestor_record_offset":14772,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":464,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":14772,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":464,"record_offset":15480,"parent_record_offset":15444,"ancestor_record_offset":14772,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":464,"record_offset":15620,"parent_record_offset":15588,"ancestor_record_offset":14772,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":464,"record_offset":15588,"parent_record_offset":15552,"ancestor_record_offset":14772,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":464,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":14772,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":464,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":14772,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":464,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":14772,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":464,"record_offset":16092,"parent_record_offset":15552,"ancestor_record_offset":14772,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15160,"parent_record_offset":15128,"ancestor_record_offset":14772,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15128,"parent_record_offset":15100,"ancestor_record_offset":14772,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15284,"parent_record_offset":15252,"ancestor_record_offset":14772,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15340,"parent_record_offset":15284,"ancestor_record_offset":14772,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15252,"parent_record_offset":15224,"ancestor_record_offset":14772,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15100,"parent_record_offset":15056,"ancestor_record_offset":14772,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":464,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":14772,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":464,"record_offset":15652,"parent_record_offset":15620,"ancestor_record_offset":14772,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":464,"record_offset":15936,"parent_record_offset":15652,"ancestor_record_offset":14772,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":464,"record_offset":15908,"parent_record_offset":15652,"ancestor_record_offset":14772,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":464,"record_offset":15712,"parent_record_offset":15652,"ancestor_record_offset":14772,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":464,"record_offset":24156,"parent_record_offset":24128,"ancestor_record_offset":14772,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":464,"record_offset":33364,"parent_record_offset":33328,"ancestor_record_offset":14772,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":464,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":14772,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":464,"record_offset":28836,"parent_record_offset":28776,"ancestor_record_offset":14772,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":464,"record_offset":28868,"parent_record_offset":28740,"ancestor_record_offset":14772,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":464,"record_offset":23884,"parent_record_offset":23860,"ancestor_record_offset":14772,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":464,"record_offset":23908,"parent_record_offset":23884,"ancestor_record_offset":14772,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":464,"record_offset":23572,"parent_record_offset":23544,"ancestor_record_offset":14772,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":464,"record_offset":23604,"parent_record_offset":23572,"ancestor_record_offset":14772,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":464,"record_offset":23544,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":464,"record_offset":23404,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":464,"record_offset":23984,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":464,"record_offset":23488,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":464,"record_offset":23348,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":464,"record_offset":23836,"parent_record_offset":23812,"ancestor_record_offset":14772,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":464,"record_offset":23184,"parent_record_offset":23160,"ancestor_record_offset":14772,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":464,"record_offset":23812,"parent_record_offset":23704,"ancestor_record_offset":14772,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":464,"record_offset":23216,"parent_record_offset":23184,"ancestor_record_offset":14772,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23752,"parent_record_offset":23728,"ancestor_record_offset":14772,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":24076,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23728,"parent_record_offset":23704,"ancestor_record_offset":14772,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23268,"parent_record_offset":23244,"ancestor_record_offset":14772,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23244,"parent_record_offset":23184,"ancestor_record_offset":14772,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23292,"parent_record_offset":23268,"ancestor_record_offset":14772,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23376,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":464,"record_offset":23704,"parent_record_offset":23292,"ancestor_record_offset":14772,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":16492,"parent_record_offset":16456,"ancestor_record_offset":14772,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":29260,"parent_record_offset":29228,"ancestor_record_offset":14772,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":29712,"parent_record_offset":29508,"ancestor_record_offset":14772,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":464,"record_offset":16632,"parent_record_offset":16604,"ancestor_record_offset":14772,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":16704,"parent_record_offset":16564,"ancestor_record_offset":14772,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":464,"record_offset":16908,"parent_record_offset":16704,"ancestor_record_offset":14772,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":464,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":14772,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":464,"record_offset":16660,"parent_record_offset":16632,"ancestor_record_offset":14772,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":464,"record_offset":16564,"parent_record_offset":16528,"ancestor_record_offset":14772,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":464,"record_offset":16528,"parent_record_offset":16492,"ancestor_record_offset":14772,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::wooden_frame_mesh_atlas_part_idx
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 616
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B72
// 首条位置: {"module":464,"inlinee":27506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":23072,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22976,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":26403,"source_line":1639,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22912,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":26401,"source_line":3806,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22884,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":26400,"source_line":2300,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22856,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":23008,"parent_record_offset":22976,"ancestor_record_offset":22812,"inlinee":26404,"source_line":1751,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":22812,"inlinee":26402,"source_line":3775,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::ilog2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":464,"record_offset":23036,"parent_record_offset":23008,"ancestor_record_offset":22812,"inlinee":14770,"source_line":1790,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::densify_railing_points
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 630
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6A
// 首条位置: {"module":464,"inlinee":27498}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2960,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3396,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2480,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":4780,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":3616,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":4752,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":6256,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4720,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4096,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":3920,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::catmull_rom_vec3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":464,"record_offset":4808,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":27499,"source_line":686,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2324,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2324,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":2324,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":2324,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2324,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":2324,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":2828,"parent_record_offset":2632,"ancestor_record_offset":2324,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":6420,"parent_record_offset":6328,"ancestor_record_offset":2324,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":2324,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2560,"parent_record_offset":2520,"ancestor_record_offset":2324,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":2324,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":2324,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":2324,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2324,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":2324,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2596,"parent_record_offset":2560,"ancestor_record_offset":2324,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":2324,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":2668,"parent_record_offset":2632,"ancestor_record_offset":2324,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":2324,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":464,"record_offset":3032,"parent_record_offset":2996,"ancestor_record_offset":2324,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":2324,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":3424,"parent_record_offset":3396,"ancestor_record_offset":2324,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":2520,"parent_record_offset":2480,"ancestor_record_offset":2324,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":464,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":2324,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":464,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":2324,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":464,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":2324,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":464,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":2324,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":464,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":2324,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":2324,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":2788,"parent_record_offset":2724,"ancestor_record_offset":2324,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":6388,"parent_record_offset":6356,"ancestor_record_offset":2324,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":5252,"parent_record_offset":5220,"ancestor_record_offset":2324,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":4892,"parent_record_offset":4860,"ancestor_record_offset":2324,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":2324,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":464,"record_offset":3328,"parent_record_offset":3032,"ancestor_record_offset":2324,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":2324,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":2324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":3580,"parent_record_offset":3424,"ancestor_record_offset":2324,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":2324,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2324,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4156,"parent_record_offset":4124,"ancestor_record_offset":2324,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":2324,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5864,"parent_record_offset":4808,"ancestor_record_offset":2324,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4860,"parent_record_offset":4808,"ancestor_record_offset":2324,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5220,"parent_record_offset":4808,"ancestor_record_offset":2324,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5040,"parent_record_offset":4808,"ancestor_record_offset":2324,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":464,"record_offset":4244,"parent_record_offset":4212,"ancestor_record_offset":2324,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 672
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6C
// 首条位置: {"module":464,"inlinee":27500}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":464,"record_offset":6880,"parent_record_offset":6828,"ancestor_record_offset":6828,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":6964,"parent_record_offset":6828,"ancestor_record_offset":6828,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":6828,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":6828,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":6828,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":464,"record_offset":7128,"parent_record_offset":7036,"ancestor_record_offset":6828,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":7064,"parent_record_offset":7036,"ancestor_record_offset":6828,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":6828,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":464,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6828,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":464,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":6828,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":464,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":6828,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":464,"record_offset":7096,"parent_record_offset":7064,"ancestor_record_offset":6828,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":464,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6828,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 679
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x139A7
// 首条位置: {"module":1545,"inlinee":80295}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1545,"record_offset":68056,"parent_record_offset":67928,"ancestor_record_offset":67928,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1545,"record_offset":68024,"parent_record_offset":67928,"ancestor_record_offset":67928,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1545,"record_offset":67968,"parent_record_offset":67928,"ancestor_record_offset":67928,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1545,"record_offset":67992,"parent_record_offset":67968,"ancestor_record_offset":67928,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1545,"record_offset":68092,"parent_record_offset":68056,"ancestor_record_offset":67928,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1545,"record_offset":68128,"parent_record_offset":68092,"ancestor_record_offset":67928,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::catmull_rom_vec3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6B
// 首条位置: {"module":464,"inlinee":27499}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5864,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":4860,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5220,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":464,"record_offset":5040,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":5252,"parent_record_offset":5220,"ancestor_record_offset":4808,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":4892,"parent_record_offset":4860,"ancestor_record_offset":4808,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":464,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":4808,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00464.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_assemble_wood_misc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_assemble_wood_misc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_assemble_wood_misc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece::railing
// 信息 ID: cace727f604534a59512cbd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=10300; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=13324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers
// 信息 ID: dc920f45b06e321284ac91ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x369dfb -> 0x8aa220 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36b2e9 -> 0x8aa220 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37d4ac -> 0x8aa220 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37c7dc -> 0x8aa220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::catmull_rom_vec3
// 信息 ID: 2899fa931406904dc515ab34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=4808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::construct_wooden_stair_steps_impl
// 信息 ID: c787ec3a322ed5149734b4b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x369f8a -> 0x8ade60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36b210 -> 0x8ade60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37d63d -> 0x8ade60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37c706 -> 0x8ade60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len
// 信息 ID: 88d4ac5ca544edaf6affdc0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=6828; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=9580; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=12604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len::closure$0
// 信息 ID: 758cf0f343accb4d88bb7be7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::decimate_to_max_len::closure_env$0> >; module=1545, inline_record=67928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::densify_railing_points
// 信息 ID: a082641a135e508eb54d14a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=2324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails
// 信息 ID: 18268faa2d4ee4b025ea953a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells; module=466, inline_record=20492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure$0
// 信息 ID: 2538231cd316924a9374fa3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=9036; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=12144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure$1
// 信息 ID: 3d73174a0bdde4effa9246f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=9552; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::SweptWoodPiece,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,alloc::alloc::Global>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$0>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_railing_rails::closure_env$1> >; module=1545, inline_record=12576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces
// 信息 ID: d4b081f78163f374daf96b89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers @ 0x8aad7b -> 0x8abb40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::queue_wooden_railing_cells @ 0x8b4ed9 -> 0x8abb40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces::closure$0
// 信息 ID: 472b177ffb36d68c38b5951a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces; module=464, inline_record=12416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points
// 信息 ID: 0a0290444e33345088badba4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=12140; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=1856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure$0
// 信息 ID: 51d710e0fda15c1676607f21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$1> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points::closure_env$0> > > >; module=1600, inline_record=6396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stringer_side_points
// 信息 ID: 831f4ddfcecbba51e272ff90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=1756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples
// 信息 ID: e2916faf07885356e10849ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84aca8 -> 0x8ab260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84b21f -> 0x8ab260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::queue_swept_wood_pieces @ 0x8abc33 -> 0x8ab260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$0
// 信息 ID: f5a725fefec2518559f037c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples; module=464, inline_record=10668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples::closure$1
// 信息 ID: 0034aabdec111df66fec55b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples; module=464, inline_record=11228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::upload_wooden_frames_to_gpu
// 信息 ID: 29305d8b36daf22c628b1439
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3d5c4d758428765d0d38244e: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands)> > @ 0x28d05f -> 0x8abfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::PendingWoodenStairsProceduralPieces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::system::commands::Commands)> @ 0x67f228 -> 0x8abfa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::wooden_frame_mesh_atlas_part_idx
// 信息 ID: d6245d3c11571769af2e30b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::upload_wooden_frames_to_gpu; module=464, inline_record=22812; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::upload_wooden_frames_to_gpu; module=464, inline_record=26912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
