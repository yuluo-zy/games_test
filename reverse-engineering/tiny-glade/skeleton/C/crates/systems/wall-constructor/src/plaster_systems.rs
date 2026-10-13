// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/plaster_systems.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ee6e71e5faa499081617d59bab427fb864df58fe81f81f5e4a6ca54727d5572d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: plaster_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 404, 1455, 1461, 1464, 1482, 1499, 1500, 3159, 4370, 4371, 4375
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":404,"record_offset":31708,"rva":8434592,"va":5377143712}
// 编译实例 RVAs: 0x80b3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1467718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e8ba9ae9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377146417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39913938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e8d2096102","size":5,"soft_fail":false,"target_va":5417057984,"va":5377144041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t39476400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e8b05c5a02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377145835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4053187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e8c3d83d00","size":5,"soft_fail":false,"target_va":5381197120,"va":5377143928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4053592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e858da3d00","size":5,"soft_fail":false,"target_va":5381197472,"va":5377143875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4090707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e8536b3e00","size":5,"soft_fail":false,"target_va":5381234608,"va":5377143896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smallvec::SmallVec$LT$A$GT$::reserve_one_unchecked
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39570225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e831cb5b02","size":5,"soft_fail":false,"target_va":5416716128,"va":5377145898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6164890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377143712,"raw_hex":"e89a115e00","size":5,"soft_fail":false,"target_va":5383309120,"va":5377144225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":31876,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":34956,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":32176,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":26280,"source_line":779,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":32716,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":26281,"source_line":1114,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::WallSide::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":31936,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":26279,"source_line":876,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":404,"record_offset":32204,"parent_record_offset":31708,"ancestor_record_offset":31708,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":35268,"parent_record_offset":35244,"ancestor_record_offset":31708,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":35292,"parent_record_offset":35268,"ancestor_record_offset":31708,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":35316,"parent_record_offset":35292,"ancestor_record_offset":31708,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":35244,"parent_record_offset":35160,"ancestor_record_offset":31708,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":35188,"parent_record_offset":35160,"ancestor_record_offset":31708,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":35160,"parent_record_offset":35136,"ancestor_record_offset":31708,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":35136,"parent_record_offset":35112,"ancestor_record_offset":31708,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":32116,"parent_record_offset":31936,"ancestor_record_offset":31708,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":31708,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":31708,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":404,"record_offset":32028,"parent_record_offset":32004,"ancestor_record_offset":31708,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":404,"record_offset":35064,"parent_record_offset":34980,"ancestor_record_offset":31708,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":35212,"parent_record_offset":35188,"ancestor_record_offset":31708,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":404,"record_offset":32260,"parent_record_offset":32236,"ancestor_record_offset":31708,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":404,"record_offset":32236,"parent_record_offset":32204,"ancestor_record_offset":31708,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":35088,"parent_record_offset":35064,"ancestor_record_offset":31708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":32820,"parent_record_offset":32716,"ancestor_record_offset":31708,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":32788,"parent_record_offset":32764,"ancestor_record_offset":31708,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":32764,"parent_record_offset":32716,"ancestor_record_offset":31708,"inlinee":15353,"source_line":984,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":32856,"parent_record_offset":32716,"ancestor_record_offset":31708,"inlinee":4829,"source_line":659,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":404,"record_offset":34980,"parent_record_offset":34956,"ancestor_record_offset":31708,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":404,"record_offset":32004,"parent_record_offset":31936,"ancestor_record_offset":31708,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":31904,"parent_record_offset":31876,"ancestor_record_offset":31708,"inlinee":26277,"source_line":913,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":404,"record_offset":31976,"parent_record_offset":31936,"ancestor_record_offset":31708,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":404,"record_offset":8312,"rva":8426960,"va":5377136080}
// 编译实例 RVAs: 0x8095d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39474842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e89a565a02","size":5,"soft_fail":false,"target_va":5416616112,"va":5377141265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39474927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8ef565a02","size":5,"soft_fail":false,"target_va":5416616304,"va":5377141372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-1458705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8efbde9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377137404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7903188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e82c6887ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5377143631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7878270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e882c987ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5377143385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce3b0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4437624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e88849bcff","size":5,"soft_fail":false,"target_va":5372699568,"va":5377137187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3658130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e86e2ec8ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377137165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t981520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e810fa0e00","size":5,"soft_fail":false,"target_va":5378120480,"va":5377138955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098aae0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1571570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8f2fa1700","size":5,"soft_fail":false,"target_va":5378714336,"va":5377142761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2371780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8c4302400","size":5,"soft_fail":false,"target_va":5379510928,"va":5377139143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4e290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4472332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e80c3e4400","size":5,"soft_fail":false,"target_va":5381612176,"va":5377139839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6186523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e81b665e00","size":5,"soft_fail":false,"target_va":5383325056,"va":5377138528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12512870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e866eebe00","size":5,"soft_fail":false,"target_va":5389650880,"va":5377138005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fb180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12521335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8770fbf00","size":5,"soft_fail":false,"target_va":5389660544,"va":5377139204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12592085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8d523c000","size":5,"soft_fail":false,"target_va":5389733232,"va":5377141142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12631255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8d7bcc000","size":5,"soft_fail":false,"target_va":5389770000,"va":5377138740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141463410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12946288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8708bc500","size":5,"soft_fail":false,"target_va":5390087184,"va":5377140891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146ca20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12984182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8761fc600","size":5,"soft_fail":false,"target_va":5390125600,"va":5377141413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39211878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e866535602","size":5,"soft_fail":false,"target_va":5416350496,"va":5377138613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39211899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e87b535602","size":5,"soft_fail":false,"target_va":5416350720,"va":5377138816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39913464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8f8076102","size":5,"soft_fail":false,"target_va":5417057072,"va":5377143603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7281345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8c11a6f00","size":5,"soft_fail":false,"target_va":5384418976,"va":5377137626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39479922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8726a5a02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377141481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39480676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8646d5a02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377140919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39480270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8ce6b5a02","size":5,"soft_fail":false,"target_va":5416621632,"va":5377141357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13008544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8a07ec600","size":5,"soft_fail":false,"target_va":5390150320,"va":5377141771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e824280000","size":5,"soft_fail":false,"target_va":5377146512,"va":5377136231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39479619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e843695a02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377141752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3765751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8f7753900","size":5,"soft_fail":false,"target_va":5380904528,"va":5377138772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4096113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e871803e00","size":5,"soft_fail":false,"target_va":5381234608,"va":5377138490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4093743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e82f773e00","size":5,"soft_fail":false,"target_va":5381232176,"va":5377138428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4469781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e815344400","size":5,"soft_fail":false,"target_va":5381609616,"va":5377139830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2458810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8ba842500","size":5,"soft_fail":false,"target_va":5379597264,"va":5377138449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6827209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8c92c6800","size":5,"soft_fail":false,"target_va":5383963792,"va":5377136578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6835716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8044e6800","size":5,"soft_fail":false,"target_va":5383972224,"va":5377136503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6823887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8cf1f6800","size":5,"soft_fail":false,"target_va":5383964912,"va":5377141020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6826615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8772a6800","size":5,"soft_fail":false,"target_va":5383962816,"va":5377136196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39478415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e88f645a02","size":5,"soft_fail":false,"target_va":5416619696,"va":5377141276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39555512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8b8915b02","size":5,"soft_fail":false,"target_va":5416697008,"va":5377141491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39477843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e853625a02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377141512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6140082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377136080,"raw_hex":"e8b2b05d00","size":5,"soft_fail":false,"target_va":5383278464,"va":5377138377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":22768,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8431,"source_line":247,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":29892,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":19900,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":22948,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":404,"record_offset":24044,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":404,"record_offset":13656,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":13308,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":404,"record_offset":12596,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":404,"record_offset":19364,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":15780,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":404,"record_offset":19276,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":404,"record_offset":13864,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":404,"record_offset":18272,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":17924,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":17868,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":19304,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":28608,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$10::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":13836,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":9876,"source_line":2322,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":15852,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":19804,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":9976,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":8572,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":8964,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":18300,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":19776,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":18008,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":13716,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":20852,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":19430,"source_line":807,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":20112,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":10784,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":26265,"source_line":188,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":8312,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":8312,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":14956,"parent_record_offset":14932,"ancestor_record_offset":8312,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":14884,"parent_record_offset":14796,"ancestor_record_offset":8312,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":19584,"parent_record_offset":19552,"ancestor_record_offset":8312,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":14824,"parent_record_offset":14796,"ancestor_record_offset":8312,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":8312,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":8312,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":8312,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":404,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":8312,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":22804,"parent_record_offset":22768,"ancestor_record_offset":8312,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":19552,"parent_record_offset":19512,"ancestor_record_offset":8312,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":8312,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":29924,"parent_record_offset":29892,"ancestor_record_offset":8312,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11604,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":19484,"parent_record_offset":19448,"ancestor_record_offset":8312,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":8312,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11488,"parent_record_offset":10784,"ancestor_record_offset":8312,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11524,"parent_record_offset":11488,"ancestor_record_offset":8312,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11940,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":17948,"parent_record_offset":17924,"ancestor_record_offset":8312,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":8312,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11632,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":8312,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":30044,"parent_record_offset":29924,"ancestor_record_offset":8312,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":12912,"parent_record_offset":12684,"ancestor_record_offset":8312,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":404,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":8312,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":404,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":8312,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":404,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":8312,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":404,"record_offset":13684,"parent_record_offset":13656,"ancestor_record_offset":8312,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":404,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":8312,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":8312,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":13336,"parent_record_offset":13308,"ancestor_record_offset":8312,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":8312,"inlinee":11539,"source_line":90,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":404,"record_offset":12660,"parent_record_offset":12620,"ancestor_record_offset":8312,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":404,"record_offset":19448,"parent_record_offset":19388,"ancestor_record_offset":8312,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":15812,"parent_record_offset":15780,"ancestor_record_offset":8312,"inlinee":11375,"source_line":262,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":404,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":8312,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":8312,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":8312,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10252,"parent_record_offset":10224,"ancestor_record_offset":8312,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10276,"parent_record_offset":10252,"ancestor_record_offset":8312,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10392,"parent_record_offset":10276,"ancestor_record_offset":8312,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":10504,"parent_record_offset":10480,"ancestor_record_offset":8312,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":30068,"parent_record_offset":30044,"ancestor_record_offset":8312,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":18952,"parent_record_offset":18928,"ancestor_record_offset":8312,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":8312,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":8312,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":18600,"parent_record_offset":18576,"ancestor_record_offset":8312,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":404,"record_offset":11688,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":19020,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":8312,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":8312,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":13368,"parent_record_offset":13336,"ancestor_record_offset":8312,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":8312,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":404,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":8312,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":404,"record_offset":14332,"parent_record_offset":13896,"ancestor_record_offset":8312,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":404,"record_offset":13896,"parent_record_offset":13864,"ancestor_record_offset":8312,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":404,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":8312,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":404,"record_offset":13944,"parent_record_offset":13896,"ancestor_record_offset":8312,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":404,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":8312,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":404,"record_offset":17972,"parent_record_offset":17948,"ancestor_record_offset":8312,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":404,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":8312,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":12868,"parent_record_offset":12840,"ancestor_record_offset":8312,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":12728,"parent_record_offset":12684,"ancestor_record_offset":8312,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":14852,"parent_record_offset":14824,"ancestor_record_offset":8312,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":21712,"parent_record_offset":21632,"ancestor_record_offset":8312,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":22056,"parent_record_offset":22028,"ancestor_record_offset":8312,"inlinee":7461,"source_line":153,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":22084,"parent_record_offset":22028,"ancestor_record_offset":8312,"inlinee":7462,"source_line":177,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":18752,"parent_record_offset":18724,"ancestor_record_offset":8312,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":18724,"parent_record_offset":18696,"ancestor_record_offset":8312,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":404,"record_offset":10124,"parent_record_offset":10084,"ancestor_record_offset":8312,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":404,"record_offset":14060,"parent_record_offset":14028,"ancestor_record_offset":8312,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":8312,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":14572,"parent_record_offset":14332,"ancestor_record_offset":8312,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":14596,"parent_record_offset":14572,"ancestor_record_offset":8312,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":13592,"parent_record_offset":13336,"ancestor_record_offset":8312,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":13448,"parent_record_offset":13368,"ancestor_record_offset":8312,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":14224,"parent_record_offset":14200,"ancestor_record_offset":8312,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":14200,"parent_record_offset":14176,"ancestor_record_offset":8312,"inlinee":10740,"source_line":2481,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":14176,"parent_record_offset":14000,"ancestor_record_offset":8312,"inlinee":10739,"source_line":2616,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":8312,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":10740,"parent_record_offset":10084,"ancestor_record_offset":8312,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":22192,"parent_record_offset":22168,"ancestor_record_offset":8312,"inlinee":7456,"source_line":612,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":12084,"parent_record_offset":11992,"ancestor_record_offset":8312,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":11912,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":10456,"parent_record_offset":10224,"ancestor_record_offset":8312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":11772,"parent_record_offset":11524,"ancestor_record_offset":8312,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":8312,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":11452,"parent_record_offset":11372,"ancestor_record_offset":8312,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":8312,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":14508,"parent_record_offset":14420,"ancestor_record_offset":8312,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":8312,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":9568,"parent_record_offset":9540,"ancestor_record_offset":8312,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":8312,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":19672,"parent_record_offset":19512,"ancestor_record_offset":8312,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":404,"record_offset":11332,"parent_record_offset":10784,"ancestor_record_offset":8312,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":8312,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":22396,"parent_record_offset":22312,"ancestor_record_offset":8312,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$40::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":19832,"parent_record_offset":19804,"ancestor_record_offset":8312,"inlinee":26259,"source_line":405,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":8312,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":9640,"parent_record_offset":9612,"ancestor_record_offset":8312,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":12784,"parent_record_offset":12684,"ancestor_record_offset":8312,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":13152,"parent_record_offset":12684,"ancestor_record_offset":8312,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":8312,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":12812,"parent_record_offset":12784,"ancestor_record_offset":8312,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":404,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":8312,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":8312,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":8312,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":8312,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":13524,"parent_record_offset":13472,"ancestor_record_offset":8312,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":9288,"parent_record_offset":9172,"ancestor_record_offset":8312,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":11372,"parent_record_offset":10784,"ancestor_record_offset":8312,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":404,"record_offset":22840,"parent_record_offset":22804,"ancestor_record_offset":8312,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":8312,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":9000,"parent_record_offset":8964,"ancestor_record_offset":8312,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":9140,"parent_record_offset":9108,"ancestor_record_offset":8312,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":9108,"parent_record_offset":9072,"ancestor_record_offset":8312,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":8312,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":8312,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":8312,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":9612,"parent_record_offset":9072,"ancestor_record_offset":8312,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8676,"parent_record_offset":8644,"ancestor_record_offset":8312,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8644,"parent_record_offset":8616,"ancestor_record_offset":8312,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8804,"parent_record_offset":8772,"ancestor_record_offset":8312,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8860,"parent_record_offset":8804,"ancestor_record_offset":8312,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8772,"parent_record_offset":8740,"ancestor_record_offset":8312,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8616,"parent_record_offset":8572,"ancestor_record_offset":8312,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":8312,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":9172,"parent_record_offset":9140,"ancestor_record_offset":8312,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":9456,"parent_record_offset":9172,"ancestor_record_offset":8312,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":9428,"parent_record_offset":9172,"ancestor_record_offset":8312,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":9232,"parent_record_offset":9172,"ancestor_record_offset":8312,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":8312,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12432,"parent_record_offset":12240,"ancestor_record_offset":8312,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":8312,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12524,"parent_record_offset":12492,"ancestor_record_offset":8312,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11156,"parent_record_offset":11124,"ancestor_record_offset":8312,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":8312,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":8312,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":8312,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":8312,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":404,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":8312,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":404,"record_offset":24008,"parent_record_offset":23984,"ancestor_record_offset":8312,"inlinee":26263,"source_line":436,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":22028,"parent_record_offset":21632,"ancestor_record_offset":8312,"inlinee":7459,"source_line":86,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":18660,"parent_record_offset":18632,"ancestor_record_offset":8312,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":18696,"parent_record_offset":18660,"ancestor_record_offset":8312,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":20384,"parent_record_offset":20232,"ancestor_record_offset":8312,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":18632,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":18492,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":18168,"parent_record_offset":18140,"ancestor_record_offset":8312,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":19048,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":18576,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":404,"record_offset":19172,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":404,"record_offset":18928,"parent_record_offset":18904,"ancestor_record_offset":8312,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":18332,"parent_record_offset":18300,"ancestor_record_offset":8312,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":8312,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":8312,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":13768,"parent_record_offset":13740,"ancestor_record_offset":8312,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":20876,"parent_record_offset":20852,"ancestor_record_offset":8312,"inlinee":26271,"source_line":917,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":18904,"parent_record_offset":18796,"ancestor_record_offset":8312,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":20136,"parent_record_offset":20112,"ancestor_record_offset":8312,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":13796,"parent_record_offset":13768,"ancestor_record_offset":8312,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":8312,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":20472,"parent_record_offset":20192,"ancestor_record_offset":8312,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":8312,"inlinee":11106,"source_line":385,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":19140,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18140,"parent_record_offset":18080,"ancestor_record_offset":8312,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":8312,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":8312,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":22120,"parent_record_offset":21600,"ancestor_record_offset":8312,"inlinee":7453,"source_line":756,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":21572,"parent_record_offset":20876,"ancestor_record_offset":8312,"inlinee":26272,"source_line":825,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":21600,"parent_record_offset":21572,"ancestor_record_offset":8312,"inlinee":7452,"source_line":817,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18404,"parent_record_offset":18372,"ancestor_record_offset":8312,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18372,"parent_record_offset":18332,"ancestor_record_offset":8312,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":8312,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18112,"parent_record_offset":18080,"ancestor_record_offset":8312,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":21632,"parent_record_offset":21600,"ancestor_record_offset":8312,"inlinee":7457,"source_line":3249,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":8312,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":8312,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":19200,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":21680,"parent_record_offset":21632,"ancestor_record_offset":8312,"inlinee":7463,"source_line":2606,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":8312,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":18796,"parent_record_offset":18436,"ancestor_record_offset":8312,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":22168,"parent_record_offset":22144,"ancestor_record_offset":8312,"inlinee":7455,"source_line":20,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":10012,"parent_record_offset":9976,"ancestor_record_offset":8312,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":29072,"parent_record_offset":29040,"ancestor_record_offset":8312,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":29528,"parent_record_offset":29324,"ancestor_record_offset":8312,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":404,"record_offset":10152,"parent_record_offset":10124,"ancestor_record_offset":8312,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":10224,"parent_record_offset":10084,"ancestor_record_offset":8312,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":10428,"parent_record_offset":10224,"ancestor_record_offset":8312,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":404,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":8312,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":404,"record_offset":11096,"parent_record_offset":11068,"ancestor_record_offset":8312,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":12240,"parent_record_offset":12192,"ancestor_record_offset":8312,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":11224,"parent_record_offset":11032,"ancestor_record_offset":8312,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":8312,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":10872,"parent_record_offset":10784,"ancestor_record_offset":8312,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":11004,"parent_record_offset":10872,"ancestor_record_offset":8312,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":12192,"parent_record_offset":12156,"ancestor_record_offset":8312,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":404,"record_offset":10912,"parent_record_offset":10872,"ancestor_record_offset":8312,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":404,"record_offset":12492,"parent_record_offset":12192,"ancestor_record_offset":8312,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":404,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":8312,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":404,"record_offset":11068,"parent_record_offset":11032,"ancestor_record_offset":8312,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":10084,"parent_record_offset":10048,"ancestor_record_offset":8312,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":10048,"parent_record_offset":10012,"ancestor_record_offset":8312,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":8312,"inlinee":26267,"source_line":193,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":8312,"inlinee":26269,"source_line":77,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":19332,"parent_record_offset":19304,"ancestor_record_offset":8312,"inlinee":26270,"source_line":115,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":22240,"parent_record_offset":20876,"ancestor_record_offset":8312,"inlinee":26273,"source_line":142,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":24588,"parent_record_offset":24564,"ancestor_record_offset":8312,"inlinee":26275,"source_line":154,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":23984,"parent_record_offset":23960,"ancestor_record_offset":8312,"inlinee":26274,"source_line":173,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669D
// 首条位置: {"module":404,"inlinee":26269}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":14132,"parent_record_offset":14108,"ancestor_record_offset":14084,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F4
// 首条位置: {"module":1499,"inlinee":79860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":32080,"parent_record_offset":31104,"ancestor_record_offset":31104,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":32140,"parent_record_offset":31104,"ancestor_record_offset":31104,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1499,"record_offset":31144,"parent_record_offset":31104,"ancestor_record_offset":31104,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":31840,"parent_record_offset":31816,"ancestor_record_offset":31104,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1499,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":31104,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1499,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":31104,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1499,"record_offset":31908,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1499,"record_offset":31644,"parent_record_offset":31616,"ancestor_record_offset":31104,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1499,"record_offset":31616,"parent_record_offset":31588,"ancestor_record_offset":31104,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":31104,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":31756,"parent_record_offset":31732,"ancestor_record_offset":31104,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$40::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1499,"record_offset":32240,"parent_record_offset":32216,"ancestor_record_offset":31104,"inlinee":26259,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":32216,"parent_record_offset":32192,"ancestor_record_offset":31104,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":32264,"parent_record_offset":32240,"ancestor_record_offset":31104,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1499,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31104,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":1499,"record_offset":31556,"parent_record_offset":31528,"ancestor_record_offset":31104,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":1499,"record_offset":31588,"parent_record_offset":31556,"ancestor_record_offset":31104,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":1499,"record_offset":31528,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1499,"record_offset":31388,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1499,"record_offset":31936,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1499,"record_offset":31472,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1499,"record_offset":31332,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1499,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":31104,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":1499,"record_offset":31172,"parent_record_offset":31144,"ancestor_record_offset":31104,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":1499,"record_offset":31792,"parent_record_offset":31688,"ancestor_record_offset":31104,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1499,"record_offset":31204,"parent_record_offset":31172,"ancestor_record_offset":31104,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31732,"parent_record_offset":31708,"ancestor_record_offset":31104,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":32028,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31708,"parent_record_offset":31688,"ancestor_record_offset":31104,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31256,"parent_record_offset":31232,"ancestor_record_offset":31104,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31232,"parent_record_offset":31172,"ancestor_record_offset":31104,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31280,"parent_record_offset":31256,"ancestor_record_offset":31104,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31360,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1499,"record_offset":31688,"parent_record_offset":31280,"ancestor_record_offset":31104,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":32108,"parent_record_offset":32080,"ancestor_record_offset":31104,"inlinee":79862,"source_line":91,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":32192,"parent_record_offset":32168,"ancestor_record_offset":31104,"inlinee":79863,"source_line":94,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F6
// 首条位置: {"module":1499,"inlinee":79862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F7
// 首条位置: {"module":1499,"inlinee":79863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":32216,"parent_record_offset":32192,"ancestor_record_offset":32192,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$40::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1499,"record_offset":32240,"parent_record_offset":32216,"ancestor_record_offset":32192,"inlinee":26259,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":32264,"parent_record_offset":32240,"ancestor_record_offset":32192,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669E
// 首条位置: {"module":404,"inlinee":26270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A1
// 首条位置: {"module":404,"inlinee":26273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":22240,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":22240,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":22396,"parent_record_offset":22312,"ancestor_record_offset":22240,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":22312,"parent_record_offset":22288,"ancestor_record_offset":22240,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":22340,"parent_record_offset":22312,"ancestor_record_offset":22240,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":404,"record_offset":22420,"parent_record_offset":22396,"ancestor_record_offset":22240,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=79
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137FB
// 首条位置: {"module":1500,"inlinee":79867}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A3
// 首条位置: {"module":404,"inlinee":26275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":24612,"parent_record_offset":24588,"ancestor_record_offset":24588,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":24636,"parent_record_offset":24612,"ancestor_record_offset":24588,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A2
// 首条位置: {"module":404,"inlinee":26274}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":404,"record_offset":24008,"parent_record_offset":23984,"ancestor_record_offset":23984,"inlinee":26263,"source_line":436,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6699
// 首条位置: {"module":404,"inlinee":26265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11488,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":12156,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":404,"record_offset":11332,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":11372,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":10872,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11604,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11524,"parent_record_offset":11488,"ancestor_record_offset":10784,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11940,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11632,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":10784,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":404,"record_offset":11688,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":10784,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":12028,"parent_record_offset":11992,"ancestor_record_offset":10784,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":12084,"parent_record_offset":11992,"ancestor_record_offset":10784,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":11912,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":11744,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":11772,"parent_record_offset":11524,"ancestor_record_offset":10784,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":11452,"parent_record_offset":11372,"ancestor_record_offset":10784,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":11420,"parent_record_offset":11372,"ancestor_record_offset":10784,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":10784,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12432,"parent_record_offset":12240,"ancestor_record_offset":10784,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10784,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12524,"parent_record_offset":12492,"ancestor_record_offset":10784,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11156,"parent_record_offset":11124,"ancestor_record_offset":10784,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10784,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":10784,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":10784,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":404,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":10784,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":404,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":10784,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":404,"record_offset":11096,"parent_record_offset":11068,"ancestor_record_offset":10784,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":12240,"parent_record_offset":12192,"ancestor_record_offset":10784,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":11224,"parent_record_offset":11032,"ancestor_record_offset":10784,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":404,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":10784,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":11004,"parent_record_offset":10872,"ancestor_record_offset":10784,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":404,"record_offset":12192,"parent_record_offset":12156,"ancestor_record_offset":10784,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":404,"record_offset":10912,"parent_record_offset":10872,"ancestor_record_offset":10784,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":404,"record_offset":12492,"parent_record_offset":12192,"ancestor_record_offset":10784,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":404,"record_offset":11068,"parent_record_offset":11032,"ancestor_record_offset":10784,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":10784,"inlinee":26267,"source_line":193,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669B
// 首条位置: {"module":404,"inlinee":26267}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::impl$0::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":280,"rva":21012704,"va":5389721824}
// 编译实例 RVAs: 0x140a0e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26893936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e8705e9a01","size":5,"soft_fail":false,"target_va":5416616112,"va":5389722171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20481928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e87878c7fe","size":5,"soft_fail":false,"target_va":5369240448,"va":5389722371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20457442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e81ed8c7fe","size":5,"soft_fail":false,"target_va":5369265120,"va":5389722557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e8262a0000","size":5,"soft_fail":false,"target_va":5389733232,"va":5389722437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26628972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e86c539601","size":5,"soft_fail":false,"target_va":5416351056,"va":5389722079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e8e94d0000","size":5,"soft_fail":false,"target_va":5389742048,"va":5389722098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26899366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e8a6739a01","size":5,"soft_fail":false,"target_va":5416621632,"va":5389722261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26897504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721824,"raw_hex":"e8606c9a01","size":5,"soft_fail":false,"target_va":5416619696,"va":5389722187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26894007","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5389721824,"raw_hex":"e9b75e9a01","size":5,"soft_fail":false,"target_va":5416616304,"va":5389722292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":980,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":448,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1468,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":1300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1461,"record_offset":1272,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":1728,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1461,"record_offset":936,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":476,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1552,"parent_record_offset":1504,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1016,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1128,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1504,"parent_record_offset":1468,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":1156,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":1404,"parent_record_offset":1332,"ancestor_record_offset":280,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":1692,"parent_record_offset":1504,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":1372,"parent_record_offset":1332,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":1664,"parent_record_offset":1504,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":1184,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1461,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":1332,"parent_record_offset":1300,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":856,"parent_record_offset":828,"ancestor_record_offset":280,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":1980,"parent_record_offset":1792,"ancestor_record_offset":280,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":548,"parent_record_offset":516,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":280,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":280,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":280,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":280,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":280,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":1792,"parent_record_offset":1756,"ancestor_record_offset":280,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":828,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":280,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":608,"parent_record_offset":476,"ancestor_record_offset":280,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":280,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":516,"parent_record_offset":476,"ancestor_record_offset":280,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":2040,"parent_record_offset":1756,"ancestor_record_offset":280,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":1461,"record_offset":672,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":404,"record_offset":280,"rva":8424960,"va":5377134080}
// 编译实例 RVAs: 0x808e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3655752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8b837c8ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377134787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t985524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8b4090f00","size":5,"soft_fail":false,"target_va":5378120480,"va":5377134951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4e290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4476958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e81e504400","size":5,"soft_fail":false,"target_va":5381612176,"va":5377135213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141463410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12951711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e89fa0c500","size":5,"soft_fail":false,"target_va":5390087184,"va":5377135468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39485786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e85a815a02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377135617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39486072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e878825a02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377135523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e830300000","size":5,"soft_fail":false,"target_va":5377146512,"va":5377134171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39485667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8e3805a02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377135704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3769520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8b0843900","size":5,"soft_fail":false,"target_va":5380904528,"va":5377135003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4474407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e827464400","size":5,"soft_fail":false,"target_va":5381609616,"va":5377135204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6829276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8dc346800","size":5,"soft_fail":false,"target_va":5383963792,"va":5377134511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6837780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e814566800","size":5,"soft_fail":false,"target_va":5383972224,"va":5377134439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6829334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e816356800","size":5,"soft_fail":false,"target_va":5383964912,"va":5377135573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6828676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e884326800","size":5,"soft_fail":false,"target_va":5383962816,"va":5377134135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39561376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e8a0a85b02","size":5,"soft_fail":false,"target_va":5416697008,"va":5377135627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39483707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377134080,"raw_hex":"e83b795a02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377135648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":3368,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":404,"record_offset":2668,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":6608,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":6692,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":6720,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":4592,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":1860,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":460,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":848,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":5636,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":3652,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":2988,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":280,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":404,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":280,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":404,"record_offset":2732,"parent_record_offset":2692,"ancestor_record_offset":280,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":404,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":280,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2188,"parent_record_offset":2160,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2216,"parent_record_offset":2188,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2136,"parent_record_offset":2108,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2276,"parent_record_offset":2160,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":404,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":280,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":4420,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":280,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":404,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":2804,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":4152,"parent_record_offset":4124,"ancestor_record_offset":280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":404,"record_offset":2008,"parent_record_offset":1968,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":404,"record_offset":3500,"parent_record_offset":3420,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":2624,"parent_record_offset":1968,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":2340,"parent_record_offset":2108,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$40::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":4620,"parent_record_offset":4592,"ancestor_record_offset":280,"inlinee":26259,"source_line":405,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":404,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":2860,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":3212,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":280,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":2888,"parent_record_offset":2860,"ancestor_record_offset":280,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":404,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":3576,"parent_record_offset":3524,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":1172,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":884,"parent_record_offset":848,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":1024,"parent_record_offset":992,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":1496,"parent_record_offset":956,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":504,"parent_record_offset":460,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":1340,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":1312,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":1116,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":404,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":280,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":404,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":280,"inlinee":26263,"source_line":436,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":4096,"parent_record_offset":4064,"ancestor_record_offset":280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":4036,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":3896,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":4448,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":3980,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":404,"record_offset":3840,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":404,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":280,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":280,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":4304,"parent_record_offset":4196,"ancestor_record_offset":280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":404,"record_offset":3708,"parent_record_offset":3676,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":4540,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":3736,"parent_record_offset":3676,"ancestor_record_offset":280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":3868,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":404,"record_offset":4196,"parent_record_offset":3784,"ancestor_record_offset":280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":1896,"parent_record_offset":1860,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":6784,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":7224,"parent_record_offset":7020,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":404,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":2108,"parent_record_offset":1968,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":2312,"parent_record_offset":2108,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":404,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":1932,"parent_record_offset":1896,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::create_plaster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":404,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":280,"inlinee":26261,"source_line":253,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6695
// 首条位置: {"module":404,"inlinee":26261}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":404,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":6632,"inlinee":26263,"source_line":436,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1499,"record_offset":280,"rva":21378064,"va":5390087184}
// 编译实例 RVAs: 0x1463410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::hot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6629133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8f3d89aff","size":5,"soft_fail":false,"target_va":5383464400,"va":5390093528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14409252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8dc2124ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390087935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t-14415621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8fb0824ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390094320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21365729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e81ffcb9fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390087916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20736120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e88897c3fe","size":5,"soft_fail":false,"target_va":5369358064,"va":5390094179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11983472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8902549ff","size":5,"soft_fail":false,"target_va":5378112288,"va":5390095755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11968801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8df5e49ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390095740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10371157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8abbf61ff","size":5,"soft_fail":false,"target_va":5379716240,"va":5390087392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6765595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8e5c398ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390090614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-354515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e82d97faff","size":5,"soft_fail":false,"target_va":5389733232,"va":5390087742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-326767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e89103fbff","size":5,"soft_fail":false,"target_va":5389769424,"va":5390096186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141423910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-269220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e85ce4fbff","size":5,"soft_fail":false,"target_va":5389826320,"va":5390095535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-232811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e89572fcff","size":5,"soft_fail":false,"target_va":5389857472,"va":5390090278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t108601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e839a80100","size":5,"soft_fail":false,"target_va":5390202048,"va":5390093442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26256367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8efa39001","size":5,"soft_fail":false,"target_va":5416350384,"va":5390094012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26264940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e86cc59001","size":5,"soft_fail":false,"target_va":5416353040,"va":5390088095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26965910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e896779b01","size":5,"soft_fail":false,"target_va":5417057664,"va":5390091749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26965689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8b9769b01","size":5,"soft_fail":false,"target_va":5417057808,"va":5390092114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26524137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8e9b99401","size":5,"soft_fail":false,"target_va":5416620769,"va":5390096627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26431101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e87d4e9301","size":5,"soft_fail":false,"target_va":5416524720,"va":5390093614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26532913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e831dc9401","size":5,"soft_fail":false,"target_va":5416620794,"va":5390087876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5869628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8c46fa6ff","size":5,"soft_fail":false,"target_va":5384226192,"va":5390095815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26524912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8f0bc9401","size":5,"soft_fail":false,"target_va":5416621408,"va":5390096491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26526394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8bac29401","size":5,"soft_fail":false,"target_va":5416621600,"va":5390095201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26533791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e89fdf9401","size":5,"soft_fail":false,"target_va":5416622208,"va":5390088412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t26525080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e898bd9401","size":5,"soft_fail":false,"target_va":5416621762,"va":5390096677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26525684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8f4bf9401","size":5,"soft_fail":false,"target_va":5416622240,"va":5390096551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26525530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e85abf9401","size":5,"soft_fail":false,"target_va":5416622320,"va":5390096785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26524779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e86bbc9401","size":5,"soft_fail":false,"target_va":5416621376,"va":5390096592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8891074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e83e5578ff","size":5,"soft_fail":false,"target_va":5381197456,"va":5390088525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8893520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8b04b78ff","size":5,"soft_fail":false,"target_va":5381195024,"va":5390088539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9184995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e81dd973ff","size":5,"soft_fail":false,"target_va":5380902448,"va":5390087438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6126119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8d985a2ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390089906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6117604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e81ca7a2ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390089823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6127502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e87280a2ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390092409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6126784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e84083a2ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390089595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6605828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8fc339bff","size":5,"soft_fail":false,"target_va":5383487568,"va":5390093391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26601917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8bde99501","size":5,"soft_fail":false,"target_va":5416697008,"va":5390095086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-329585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e88ff8faff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390100716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26524172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e80cba9401","size":5,"soft_fail":false,"target_va":5416619360,"va":5390095183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::plaster_systems::split_wall_into_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12944882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e80e7a3aff","size":5,"soft_fail":false,"target_va":5377143712,"va":5390088589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6785795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8fd7498ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390088030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::half_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6785650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e88e7598ff","size":5,"soft_fail":false,"target_va":5383302400,"va":5390088045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6786101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8cb7398ff","size":5,"soft_fail":false,"target_va":5383304736,"va":5390090832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6777143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8c99698ff","size":5,"soft_fail":false,"target_va":5383313440,"va":5390090578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6810831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e8311398ff","size":5,"soft_fail":false,"target_va":5383282224,"va":5390093050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6808995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390087184,"raw_hex":"e85d1a98ff","size":5,"soft_fail":false,"target_va":5383282208,"va":5390091198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rdi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5390087184,"raw_hex":"ffd7","size":2,"soft_fail":false,"va":5390093575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1499,"record_offset":27808,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1499,"record_offset":27624,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":904,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":28016,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1499,"record_offset":27892,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1499,"record_offset":28940,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1499,"record_offset":28268,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1499,"record_offset":28240,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":960,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":28568,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1499,"record_offset":29000,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::generate_plaster_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":2484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79854,"source_line":341,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4412,"parent_record_offset":4228,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5600,"parent_record_offset":5508,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1499,"record_offset":27652,"parent_record_offset":27624,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1499,"record_offset":29372,"parent_record_offset":29348,"ancestor_record_offset":280,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":17368,"parent_record_offset":17336,"ancestor_record_offset":280,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4228,"parent_record_offset":4192,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1499,"record_offset":27832,"parent_record_offset":27808,"ancestor_record_offset":280,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3484,"parent_record_offset":3444,"ancestor_record_offset":280,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1588,"parent_record_offset":1556,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3416,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":280,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":27856,"parent_record_offset":27832,"ancestor_record_offset":280,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":7024,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":18268,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":18300,"parent_record_offset":18268,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4928,"parent_record_offset":4592,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5140,"parent_record_offset":4592,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4108,"parent_record_offset":3832,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":10924,"parent_record_offset":10892,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":19448,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4544,"parent_record_offset":3832,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":30204,"parent_record_offset":30180,"ancestor_record_offset":280,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1499,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1499,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1499,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":280,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1499,"record_offset":18232,"parent_record_offset":18176,"ancestor_record_offset":280,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1499,"record_offset":18204,"parent_record_offset":18176,"ancestor_record_offset":280,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1499,"record_offset":1344,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1499,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1108,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":280,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":280,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":280,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":280,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":1764,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":280,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2292,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":280,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1499,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":280,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":280,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":280,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":280,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1840,"parent_record_offset":1812,"ancestor_record_offset":280,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":280,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":280,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1532,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1499,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":280,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":280,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1499,"record_offset":28328,"parent_record_offset":28300,"ancestor_record_offset":280,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1499,"record_offset":28356,"parent_record_offset":28328,"ancestor_record_offset":280,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1499,"record_offset":28300,"parent_record_offset":28268,"ancestor_record_offset":280,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1499,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":280,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1499,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":280,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1499,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1499,"record_offset":16864,"parent_record_offset":16824,"ancestor_record_offset":280,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1499,"record_offset":16896,"parent_record_offset":16864,"ancestor_record_offset":280,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1499,"record_offset":17040,"parent_record_offset":16928,"ancestor_record_offset":280,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1499,"record_offset":1380,"parent_record_offset":1344,"ancestor_record_offset":280,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9056,"parent_record_offset":9028,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9084,"parent_record_offset":9056,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":8996,"parent_record_offset":8964,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9028,"parent_record_offset":8996,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9144,"parent_record_offset":9028,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":13288,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":13316,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":20496,"parent_record_offset":20460,"ancestor_record_offset":280,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":12092,"parent_record_offset":11932,"ancestor_record_offset":280,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":6904,"parent_record_offset":6872,"ancestor_record_offset":280,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":17740,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":9996,"parent_record_offset":9772,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11772,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":280,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":280,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1499,"record_offset":4372,"parent_record_offset":4316,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1499,"record_offset":16824,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":280,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":280,"inlinee":37824,"source_line":118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":4636,"parent_record_offset":4592,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":3940,"parent_record_offset":3832,"ancestor_record_offset":280,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":6164,"parent_record_offset":6084,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":20460,"parent_record_offset":20432,"ancestor_record_offset":280,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":20432,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":11980,"parent_record_offset":11932,"ancestor_record_offset":280,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":18572,"parent_record_offset":18516,"ancestor_record_offset":280,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5180,"parent_record_offset":3832,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1499,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":280,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1499,"record_offset":6248,"parent_record_offset":6164,"ancestor_record_offset":280,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1499,"record_offset":17152,"parent_record_offset":16928,"ancestor_record_offset":280,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6336,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":15785,"source_line":3228,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":280,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6872,"parent_record_offset":6804,"ancestor_record_offset":280,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6392,"parent_record_offset":6364,"ancestor_record_offset":280,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6012,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":22760,"parent_record_offset":22736,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1499,"record_offset":22876,"parent_record_offset":22848,"ancestor_record_offset":280,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1499,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":280,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1499,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":6292,"parent_record_offset":6084,"ancestor_record_offset":280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":280,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":280,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":280,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9740,"parent_record_offset":9632,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9684,"parent_record_offset":9632,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":20808,"parent_record_offset":20780,"ancestor_record_offset":280,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":20984,"parent_record_offset":20956,"ancestor_record_offset":280,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":1976,"parent_record_offset":1916,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":280,"inlinee":5089,"source_line":49,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":16980,"parent_record_offset":16928,"ancestor_record_offset":280,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":16928,"parent_record_offset":16896,"ancestor_record_offset":280,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":280,"inlinee":8703,"source_line":220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":10748,"parent_record_offset":10716,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":20528,"parent_record_offset":20460,"ancestor_record_offset":280,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":280,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":1216,"parent_record_offset":1132,"ancestor_record_offset":280,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":2040,"parent_record_offset":1864,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":3736,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":1316,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":9896,"parent_record_offset":9856,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":5888,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":9492,"parent_record_offset":8816,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":280,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":280,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":3624,"parent_record_offset":3444,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":280,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4792,"parent_record_offset":4736,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":280,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":27956,"parent_record_offset":27892,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5344,"parent_record_offset":5284,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1499,"record_offset":8520,"parent_record_offset":8492,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":8492,"parent_record_offset":8464,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":7056,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":19324,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19560,"parent_record_offset":19520,"ancestor_record_offset":280,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":10120,"parent_record_offset":10084,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19484,"parent_record_offset":19448,"ancestor_record_offset":280,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":10148,"parent_record_offset":10120,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19520,"parent_record_offset":19484,"ancestor_record_offset":280,"inlinee":11933,"source_line":579,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1499,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8548,"parent_record_offset":8520,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":280,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":280,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":280,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8140,"parent_record_offset":8024,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":19600,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1499,"record_offset":7852,"parent_record_offset":7816,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1499,"record_offset":7992,"parent_record_offset":7960,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1499,"record_offset":7960,"parent_record_offset":7924,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":8464,"parent_record_offset":7924,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1499,"record_offset":8636,"parent_record_offset":8608,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1499,"record_offset":8608,"parent_record_offset":7816,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8308,"parent_record_offset":8024,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8280,"parent_record_offset":8024,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8084,"parent_record_offset":8024,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1499,"record_offset":5740,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1499,"record_offset":3660,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":280,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":280,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1499,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":280,"inlinee":11809,"source_line":161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1499,"record_offset":12684,"parent_record_offset":12656,"ancestor_record_offset":280,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":280,"inlinee":26282,"source_line":380,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12856,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26742,"source_line":1081,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12656,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11520,"parent_record_offset":11484,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11484,"parent_record_offset":11444,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13904,"parent_record_offset":13736,"ancestor_record_offset":280,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11420,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11392,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12568,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12304,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12392,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":21956,"source_line":1444,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12128,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12216,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11224,"parent_record_offset":11192,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11192,"parent_record_offset":11160,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":21520,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":17436,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":20956,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":18004,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":20780,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11020,"parent_record_offset":10892,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1499,"record_offset":9632,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1499,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":280,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1499,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8680,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8724,"parent_record_offset":8680,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":14228,"parent_record_offset":14184,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":14728,"parent_record_offset":14512,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1499,"record_offset":8892,"parent_record_offset":8864,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8964,"parent_record_offset":8816,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":9180,"parent_record_offset":8964,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":7816,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1499,"record_offset":18152,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1499,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":280,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7736,"parent_record_offset":7696,"ancestor_record_offset":280,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7568,"parent_record_offset":7536,"ancestor_record_offset":280,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7664,"parent_record_offset":7536,"ancestor_record_offset":280,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":280,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":280,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":26984,"parent_record_offset":26928,"ancestor_record_offset":280,"inlinee":4829,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5916,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":280,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":280,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7696,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7536,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":280,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13624,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11256,"parent_record_offset":11224,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1499,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1499,"record_offset":8816,"parent_record_offset":8768,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1499,"record_offset":8768,"parent_record_offset":8724,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":3788,"parent_record_offset":3736,"ancestor_record_offset":280,"inlinee":79855,"source_line":356,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":280,"inlinee":79858,"source_line":364,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":280,"inlinee":79859,"source_line":365,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":6204,"parent_record_offset":6164,"ancestor_record_offset":280,"inlinee":79640,"source_line":388,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":6516,"parent_record_offset":6484,"ancestor_record_offset":280,"inlinee":79639,"source_line":404,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":996,"parent_record_offset":960,"ancestor_record_offset":280,"inlinee":79853,"source_line":289,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":3832,"parent_record_offset":3788,"ancestor_record_offset":280,"inlinee":79856,"source_line":818,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":280,"inlinee":79321,"source_line":825,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::sd_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":13736,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26289,"source_line":993,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5284,"parent_record_offset":3832,"ancestor_record_offset":280,"inlinee":79857,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":9772,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10864,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10892,"parent_record_offset":10864,"ancestor_record_offset":280,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":3708,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10184,"parent_record_offset":10084,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":10052,"parent_record_offset":9772,"ancestor_record_offset":280,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":10024,"parent_record_offset":9772,"ancestor_record_offset":280,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9820,"parent_record_offset":9772,"ancestor_record_offset":280,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9856,"parent_record_offset":9820,"ancestor_record_offset":280,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":280,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9932,"parent_record_offset":9772,"ancestor_record_offset":280,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1499,"record_offset":10716,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10084,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1499,"record_offset":11112,"parent_record_offset":10864,"ancestor_record_offset":280,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1499,"record_offset":17772,"parent_record_offset":2484,"ancestor_record_offset":280,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137ED
// 首条位置: {"module":1499,"inlinee":79853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1499,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":996,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":996,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":996,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":996,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1588,"parent_record_offset":1556,"ancestor_record_offset":996,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":996,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1499,"record_offset":1344,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1108,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":996,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":996,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1499,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":996,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":996,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":1764,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":996,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2292,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1499,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":996,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1499,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":996,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":996,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":996,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1499,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":996,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1840,"parent_record_offset":1812,"ancestor_record_offset":996,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":996,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":996,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":1532,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1499,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":996,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1499,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":996,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1499,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":996,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1499,"record_offset":1380,"parent_record_offset":1344,"ancestor_record_offset":996,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":1976,"parent_record_offset":1916,"ancestor_record_offset":996,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":996,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":1216,"parent_record_offset":1132,"ancestor_record_offset":996,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":2040,"parent_record_offset":1864,"ancestor_record_offset":996,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":1316,"parent_record_offset":1056,"ancestor_record_offset":996,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":996,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":996,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":996,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":996,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":996,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":996,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1499,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":996,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1499,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":996,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1499,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":996,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137EE
// 首条位置: {"module":1499,"inlinee":79854}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3416,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":7024,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":18268,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":17280,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":7140,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":18720,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":19448,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":13288,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":13316,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":17740,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":21584,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11772,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1499,"record_offset":16824,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":20432,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":9536,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6336,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":15785,"source_line":3228,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6012,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":5772,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":3736,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":19352,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":5888,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":14140,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":11660,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":7056,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":19324,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":11828,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":19600,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1499,"record_offset":5740,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1499,"record_offset":3660,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12856,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26742,"source_line":1081,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12656,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11420,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11392,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12568,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12304,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12392,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":21956,"source_line":1444,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12128,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":12216,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":21520,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":17436,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":20956,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":18004,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":20780,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":21072,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1499,"record_offset":9632,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8680,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":7816,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1499,"record_offset":18152,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5916,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7696,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7536,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13624,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":20376,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::sd_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":13736,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26289,"source_line":993,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":17600,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":79857,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":9772,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10864,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":3708,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1499,"record_offset":10716,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10084,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1499,"record_offset":17772,"parent_record_offset":2484,"ancestor_record_offset":2484,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":2484,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":2484,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":2484,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":2484,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":2484,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":2484,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4412,"parent_record_offset":4228,"ancestor_record_offset":2484,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5600,"parent_record_offset":5508,"ancestor_record_offset":2484,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2484,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":2484,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":2484,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":2484,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":2484,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2484,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":17368,"parent_record_offset":17336,"ancestor_record_offset":2484,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2484,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":2484,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":2484,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4228,"parent_record_offset":4192,"ancestor_record_offset":2484,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":2484,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":2484,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":2484,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2484,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":3484,"parent_record_offset":3444,"ancestor_record_offset":2484,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":2484,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":18300,"parent_record_offset":18268,"ancestor_record_offset":2484,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4928,"parent_record_offset":4592,"ancestor_record_offset":2484,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5140,"parent_record_offset":4592,"ancestor_record_offset":2484,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4108,"parent_record_offset":3832,"ancestor_record_offset":2484,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":2484,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":10924,"parent_record_offset":10892,"ancestor_record_offset":2484,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4544,"parent_record_offset":3832,"ancestor_record_offset":2484,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1499,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":2484,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1499,"record_offset":18232,"parent_record_offset":18176,"ancestor_record_offset":2484,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1499,"record_offset":18204,"parent_record_offset":18176,"ancestor_record_offset":2484,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1499,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":2484,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1499,"record_offset":16864,"parent_record_offset":16824,"ancestor_record_offset":2484,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1499,"record_offset":16896,"parent_record_offset":16864,"ancestor_record_offset":2484,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1499,"record_offset":17040,"parent_record_offset":16928,"ancestor_record_offset":2484,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9056,"parent_record_offset":9028,"ancestor_record_offset":2484,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9084,"parent_record_offset":9056,"ancestor_record_offset":2484,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":8996,"parent_record_offset":8964,"ancestor_record_offset":2484,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9028,"parent_record_offset":8996,"ancestor_record_offset":2484,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9144,"parent_record_offset":9028,"ancestor_record_offset":2484,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1499,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":2484,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":2484,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":2484,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":2484,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":20496,"parent_record_offset":20460,"ancestor_record_offset":2484,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":12092,"parent_record_offset":11932,"ancestor_record_offset":2484,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":6904,"parent_record_offset":6872,"ancestor_record_offset":2484,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":2484,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":9996,"parent_record_offset":9772,"ancestor_record_offset":2484,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":14080,"parent_record_offset":13736,"ancestor_record_offset":2484,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":2484,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":2484,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":2484,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1499,"record_offset":4372,"parent_record_offset":4316,"ancestor_record_offset":2484,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":2484,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":2484,"inlinee":37824,"source_line":118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":2484,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":4636,"parent_record_offset":4592,"ancestor_record_offset":2484,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":3940,"parent_record_offset":3832,"ancestor_record_offset":2484,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":6164,"parent_record_offset":6084,"ancestor_record_offset":2484,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":20460,"parent_record_offset":20432,"ancestor_record_offset":2484,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":11980,"parent_record_offset":11932,"ancestor_record_offset":2484,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":18572,"parent_record_offset":18516,"ancestor_record_offset":2484,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":2484,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5180,"parent_record_offset":3832,"ancestor_record_offset":2484,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1499,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":2484,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1499,"record_offset":6248,"parent_record_offset":6164,"ancestor_record_offset":2484,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1499,"record_offset":17152,"parent_record_offset":16928,"ancestor_record_offset":2484,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":2484,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6872,"parent_record_offset":6804,"ancestor_record_offset":2484,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":6392,"parent_record_offset":6364,"ancestor_record_offset":2484,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1499,"record_offset":22760,"parent_record_offset":22736,"ancestor_record_offset":2484,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1499,"record_offset":22876,"parent_record_offset":22848,"ancestor_record_offset":2484,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1499,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":2484,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1499,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":2484,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":2484,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":2484,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":6292,"parent_record_offset":6084,"ancestor_record_offset":2484,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":2484,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":2484,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":2484,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1499,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":2484,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9740,"parent_record_offset":9632,"ancestor_record_offset":2484,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9684,"parent_record_offset":9632,"ancestor_record_offset":2484,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":20808,"parent_record_offset":20780,"ancestor_record_offset":2484,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":20984,"parent_record_offset":20956,"ancestor_record_offset":2484,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":2484,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":8864,"parent_record_offset":8816,"ancestor_record_offset":2484,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":2484,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":2484,"inlinee":5089,"source_line":49,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":16980,"parent_record_offset":16928,"ancestor_record_offset":2484,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":16928,"parent_record_offset":16896,"ancestor_record_offset":2484,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1499,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":2484,"inlinee":8703,"source_line":220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":2484,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":10748,"parent_record_offset":10716,"ancestor_record_offset":2484,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1499,"record_offset":20528,"parent_record_offset":20460,"ancestor_record_offset":2484,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":4836,"parent_record_offset":4636,"ancestor_record_offset":2484,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":9896,"parent_record_offset":9856,"ancestor_record_offset":2484,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":9492,"parent_record_offset":8816,"ancestor_record_offset":2484,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":2484,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":2484,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":3624,"parent_record_offset":3444,"ancestor_record_offset":2484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":2484,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":2484,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4792,"parent_record_offset":4736,"ancestor_record_offset":2484,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":2484,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":2484,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":2484,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":2484,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":2484,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5344,"parent_record_offset":5284,"ancestor_record_offset":2484,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1499,"record_offset":8520,"parent_record_offset":8492,"ancestor_record_offset":2484,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1499,"record_offset":8492,"parent_record_offset":8464,"ancestor_record_offset":2484,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19560,"parent_record_offset":19520,"ancestor_record_offset":2484,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":10120,"parent_record_offset":10084,"ancestor_record_offset":2484,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19484,"parent_record_offset":19448,"ancestor_record_offset":2484,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":10148,"parent_record_offset":10120,"ancestor_record_offset":2484,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1499,"record_offset":19520,"parent_record_offset":19484,"ancestor_record_offset":2484,"inlinee":11933,"source_line":579,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1499,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":2484,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":2484,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":2484,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8548,"parent_record_offset":8520,"ancestor_record_offset":2484,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":2484,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":2484,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":2484,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":2484,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":8140,"parent_record_offset":8024,"ancestor_record_offset":2484,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1499,"record_offset":7852,"parent_record_offset":7816,"ancestor_record_offset":2484,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1499,"record_offset":7992,"parent_record_offset":7960,"ancestor_record_offset":2484,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1499,"record_offset":7960,"parent_record_offset":7924,"ancestor_record_offset":2484,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":2484,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":2484,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":2484,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1499,"record_offset":8464,"parent_record_offset":7924,"ancestor_record_offset":2484,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1499,"record_offset":8636,"parent_record_offset":8608,"ancestor_record_offset":2484,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1499,"record_offset":8608,"parent_record_offset":7816,"ancestor_record_offset":2484,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":2484,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8308,"parent_record_offset":8024,"ancestor_record_offset":2484,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8280,"parent_record_offset":8024,"ancestor_record_offset":2484,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1499,"record_offset":8084,"parent_record_offset":8024,"ancestor_record_offset":2484,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1499,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":2484,"inlinee":11809,"source_line":161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1499,"record_offset":12684,"parent_record_offset":12656,"ancestor_record_offset":2484,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":2484,"inlinee":26282,"source_line":380,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11520,"parent_record_offset":11484,"ancestor_record_offset":2484,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11484,"parent_record_offset":11444,"ancestor_record_offset":2484,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":2484,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13904,"parent_record_offset":13736,"ancestor_record_offset":2484,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13848,"parent_record_offset":13736,"ancestor_record_offset":2484,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11224,"parent_record_offset":11192,"ancestor_record_offset":2484,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11192,"parent_record_offset":11160,"ancestor_record_offset":2484,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":2484,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":2484,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1499,"record_offset":11020,"parent_record_offset":10892,"ancestor_record_offset":2484,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8724,"parent_record_offset":8680,"ancestor_record_offset":2484,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":14228,"parent_record_offset":14184,"ancestor_record_offset":2484,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":14728,"parent_record_offset":14512,"ancestor_record_offset":2484,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1499,"record_offset":8892,"parent_record_offset":8864,"ancestor_record_offset":2484,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":8964,"parent_record_offset":8816,"ancestor_record_offset":2484,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1499,"record_offset":9180,"parent_record_offset":8964,"ancestor_record_offset":2484,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1499,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":2484,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7736,"parent_record_offset":7696,"ancestor_record_offset":2484,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7568,"parent_record_offset":7536,"ancestor_record_offset":2484,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":7664,"parent_record_offset":7536,"ancestor_record_offset":2484,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":2484,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":2484,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":26984,"parent_record_offset":26928,"ancestor_record_offset":2484,"inlinee":4829,"source_line":659,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":2484,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":2484,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1499,"record_offset":22784,"parent_record_offset":22760,"ancestor_record_offset":2484,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":2484,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":11256,"parent_record_offset":11224,"ancestor_record_offset":2484,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1499,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":2484,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1499,"record_offset":8816,"parent_record_offset":8768,"ancestor_record_offset":2484,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1499,"record_offset":8768,"parent_record_offset":8724,"ancestor_record_offset":2484,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":3788,"parent_record_offset":3736,"ancestor_record_offset":2484,"inlinee":79855,"source_line":356,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":2484,"inlinee":79858,"source_line":364,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":2484,"inlinee":79859,"source_line":365,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":6204,"parent_record_offset":6164,"ancestor_record_offset":2484,"inlinee":79640,"source_line":388,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":6516,"parent_record_offset":6484,"ancestor_record_offset":2484,"inlinee":79639,"source_line":404,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":3832,"parent_record_offset":3788,"ancestor_record_offset":2484,"inlinee":79856,"source_line":818,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":2484,"inlinee":79321,"source_line":825,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5284,"parent_record_offset":3832,"ancestor_record_offset":2484,"inlinee":79857,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10892,"parent_record_offset":10864,"ancestor_record_offset":2484,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1499,"record_offset":10184,"parent_record_offset":10084,"ancestor_record_offset":2484,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":10052,"parent_record_offset":9772,"ancestor_record_offset":2484,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":10024,"parent_record_offset":9772,"ancestor_record_offset":2484,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9820,"parent_record_offset":9772,"ancestor_record_offset":2484,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9856,"parent_record_offset":9820,"ancestor_record_offset":2484,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":2484,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1499,"record_offset":9932,"parent_record_offset":9772,"ancestor_record_offset":2484,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1499,"record_offset":11112,"parent_record_offset":10864,"ancestor_record_offset":2484,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137EF
// 首条位置: {"module":1499,"inlinee":79855}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":3832,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":79856,"source_line":818,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3788,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":3788,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":3788,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":3788,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":3788,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3788,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4412,"parent_record_offset":4228,"ancestor_record_offset":3788,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5600,"parent_record_offset":5508,"ancestor_record_offset":3788,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5052,"parent_record_offset":5012,"ancestor_record_offset":3788,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":3788,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3788,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":3788,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":3788,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3788,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3788,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3788,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4228,"parent_record_offset":4192,"ancestor_record_offset":3788,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":3788,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":3788,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":3788,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5012,"parent_record_offset":4592,"ancestor_record_offset":3788,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3788,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4928,"parent_record_offset":4592,"ancestor_record_offset":3788,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5140,"parent_record_offset":4592,"ancestor_record_offset":3788,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4108,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":3788,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4544,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1499,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3788,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1499,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3788,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":3788,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1499,"record_offset":4372,"parent_record_offset":4316,"ancestor_record_offset":3788,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":3788,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3788,"inlinee":37824,"source_line":118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":4636,"parent_record_offset":4592,"ancestor_record_offset":3788,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":3940,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":3788,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5180,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3788,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3788,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":3788,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":3788,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":4836,"parent_record_offset":4636,"ancestor_record_offset":3788,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3788,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5436,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":3788,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3788,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4792,"parent_record_offset":4736,"ancestor_record_offset":3788,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3788,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":3788,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5344,"parent_record_offset":5284,"ancestor_record_offset":3788,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":3788,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":3788,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3788,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":3788,"inlinee":79321,"source_line":825,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5284,"parent_record_offset":3832,"ancestor_record_offset":3788,"inlinee":79857,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 364
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F2
// 首条位置: {"module":1499,"inlinee":79858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 365
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F3
// 首条位置: {"module":1499,"inlinee":79859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13718
// 首条位置: {"module":1482,"inlinee":79640}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13717
// 首条位置: {"module":1482,"inlinee":79639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 818
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F0
// 首条位置: {"module":1499,"inlinee":79856}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4108,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4544,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":3940,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5180,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5436,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":5284,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":79857,"source_line":1001,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":3832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":3832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":3832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":3832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":4412,"parent_record_offset":4228,"ancestor_record_offset":3832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1499,"record_offset":5600,"parent_record_offset":5508,"ancestor_record_offset":3832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5052,"parent_record_offset":5012,"ancestor_record_offset":3832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":3832,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3832,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":3832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":3832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3832,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4228,"parent_record_offset":4192,"ancestor_record_offset":3832,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":3832,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":3832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1499,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":3832,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5012,"parent_record_offset":4592,"ancestor_record_offset":3832,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3832,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4928,"parent_record_offset":4592,"ancestor_record_offset":3832,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":5140,"parent_record_offset":4592,"ancestor_record_offset":3832,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":3832,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1499,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3832,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1499,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3832,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1499,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":3832,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1499,"record_offset":4372,"parent_record_offset":4316,"ancestor_record_offset":3832,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":3832,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1499,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3832,"inlinee":37824,"source_line":118,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1499,"record_offset":4636,"parent_record_offset":4592,"ancestor_record_offset":3832,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1499,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":3832,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3832,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3832,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1499,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":3832,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1499,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":3832,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1499,"record_offset":4836,"parent_record_offset":4636,"ancestor_record_offset":3832,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1499,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3832,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":3832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":3832,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3832,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4792,"parent_record_offset":4736,"ancestor_record_offset":3832,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3832,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1499,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":3832,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5344,"parent_record_offset":5284,"ancestor_record_offset":3832,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":3832,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":3832,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1499,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3832,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1499,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":3832,"inlinee":79321,"source_line":825,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 825
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135D9
// 首条位置: {"module":1455,"inlinee":79321}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::WallSide::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 876
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A7
// 首条位置: {"module":404,"inlinee":26279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":32116,"parent_record_offset":31936,"ancestor_record_offset":31936,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":404,"record_offset":32004,"parent_record_offset":31936,"ancestor_record_offset":31936,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":404,"record_offset":31976,"parent_record_offset":31936,"ancestor_record_offset":31936,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":404,"record_offset":32028,"parent_record_offset":32004,"ancestor_record_offset":31936,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 913
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A5
// 首条位置: {"module":404,"inlinee":26277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::sd_box
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 993
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66B1
// 首条位置: {"module":1499,"inlinee":26289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":14052,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":14080,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":26282,"source_line":380,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13932,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13904,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13848,"parent_record_offset":13736,"ancestor_record_offset":13736,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":13736,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1499,"record_offset":14012,"parent_record_offset":13988,"ancestor_record_offset":13736,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1499,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13736,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1499,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13736,"inlinee":11809,"source_line":161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1499,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":13736,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1499,"record_offset":13988,"parent_record_offset":13956,"ancestor_record_offset":13736,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 1001
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137F1
// 首条位置: {"module":1499,"inlinee":79857}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1499,"record_offset":17708,"parent_record_offset":17600,"ancestor_record_offset":17600,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1499,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1499,"record_offset":5344,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01499.jsonl.gz"}
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
// 完整结构及原始类型记录: plaster_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::PlasterBufferOwner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_wall_constructor::plaster_systems::PlasterBufferOwner,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::WallSide
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "array$<system_wall_constructor::plaster_systems::WallSide,4>", "constructor": "array$", "arguments": ["system_wall_constructor::plaster_systems::WallSide", "4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::IntoIter<array$<system_wall_constructor::plaster_systems::WallSide,4> >", "constructor": "smallvec::IntoIter", "arguments": ["array$<system_wall_constructor::plaster_systems::WallSide,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<smallvec::IntoIter<array$<system_wall_constructor::plaster_systems::WallSide,4> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["smallvec::IntoIter<array$<system_wall_constructor::plaster_systems::WallSide,4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_wall_constructor::plaster_systems::WallSide,4> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_wall_constructor::plaster_systems::WallSide,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: plaster_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: plaster_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::WallSide::new
// 信息 ID: c5fcf3ca9df27e5312cf4935
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=31936; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=32500; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=33184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=33864; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=34544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster
// 信息 ID: 80def1002af2e832177c7795
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 874b51eb6a7649c58e1b85fa: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> @ 0x1bd0b7 -> 0x808e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> > @ 0x279f79 -> 0x808e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 信息 ID: 896b4968ba295c07bb1fad84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::create_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::create_plaster; module=404, inline_record=6632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh
// 信息 ID: 60317eceb72f25fc0ad02a19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=2484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
// 信息 ID: 98202599757094f6854d850b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=3788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
// 信息 ID: 43dfb8cd8b2b529009eeff2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=5800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
// 信息 ID: 991658cda0f6b39eac825d57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=5856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
// 信息 ID: 33e33da02beb8fd3a8d94b7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=6204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
// 信息 ID: a3ac8478d3aac697b04e6230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=6516; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=6836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
// 信息 ID: 7fc7e3207881a38177cf2947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 信息 ID: 5e3fdaf03fdc85dc896b6c29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::filter::Filter -> ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities -> ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::create_plaster @ 0x80936c -> 0x1463410 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x80a89b -> 0x1463410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::impl$0::drop
// 信息 ID: 0fe25804937646e61584a3cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<system_wall_constructor::plaster_systems::PlasterBufferOwner> @ 0x140eea9 -> 0x140a0e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
// 信息 ID: 794ea9296ba9bfa6858b1fe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=10784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
// 信息 ID: cd71fdb671006bd1a7116241
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=11656; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=11824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster
// 信息 ID: 9a96458b8af9c07b658f156d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8db8701b75833c4d07278987: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1bd820 -> 0x8095d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x2888e1 -> 0x8095d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 信息 ID: 4094e0ca327971c9069e0867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::option::IntoIter<alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >,alloc::vec::into_iter::IntoIter<bevy_ecs::entity::Entity,alloc::alloc::Global>,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=14084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$1
// 信息 ID: 9430a5a4970a6a196cdbde9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::filter_map::FilterMap<bevy_ecs::message::iterators::MessageIterator<country_core::resources::walls::plaster::RecalculatePlasterCmd>,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::walls::plaster::RecalculatePlasterCmd> >,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::walls::plaster::RecalculatePlasterCmd> >,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1>; module=1499, inline_record=31104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$0
// 信息 ID: 9aead18537d0080624b00b45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::walls::plaster::RecalculatePlasterCmd> >,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1>; module=1499, inline_record=32108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$1::closure$1
// 信息 ID: 1bef2c019d22f26e0d1bf732
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::walls::plaster::RecalculatePlasterCmd> >,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$1>; module=1499, inline_record=32192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$2
// 信息 ID: a84ffa40b88a90f9eaab0c35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=19332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$3
// 信息 ID: 0b42b425ccfcdff9a750281d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=22240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$4
// 信息 ID: b937680f5f2c50a6d1a072cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::sort8_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::stable::driftsort_main<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4>,alloc::vec::Vec<country_core::resources::walls::WallId,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::stable::drift::sort<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::stable::quicksort::quicksort<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::pivot::median3_rec<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 73 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5556; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5580; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5680; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5704; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5840; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5864; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5952; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1500, inline_record=5980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$5
// 信息 ID: 76e3979e16373c01e28a96ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=24588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$6
// 信息 ID: af5f9732952ec83473fe27c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=404, inline_record=23984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 信息 ID: 3d9444123a06bd5ad60ce356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=3832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 信息 ID: 76d4145e49cc76140d9af32e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::mirror_holes_as_needed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=4884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::sd_box
// 信息 ID: cc0d2c4a6c420c224cd5d9ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=13736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides
// 信息 ID: 5ddeebd50469ebc6a42f708e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83b6b7 -> 0x80b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x146398d -> 0x80b3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 信息 ID: a56194c517297991fa61ad3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::split_wall_into_sides", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=404, inline_record=31904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 信息 ID: f75745885dff4eb4695892c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=5284; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=5376; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=5408; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1499, inline_record=17600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
