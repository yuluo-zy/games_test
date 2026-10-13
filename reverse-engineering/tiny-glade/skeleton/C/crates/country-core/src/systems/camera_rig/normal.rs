// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/camera_rig/normal.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0c25aa2f4576a76f769f8220fef8a1e82166954cbfcd52f3e52a1bd9d598647b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: normal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 549, 1643
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":87132,"rva":11271952,"va":5379981072}
// 编译实例 RVAs: 0xabff10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..f32..sse2..quat..Quat$u20$as$u20$glam..euler..FromEuler$GT$::from_euler_angles
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4629807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379981072,"raw_hex":"e82fa54600","size":5,"soft_fail":false,"target_va":5384611280,"va":5379981468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3343010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379981072,"raw_hex":"e8a2023300","size":5,"soft_fail":false,"target_va":5383325056,"va":5379982041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37076521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379981072,"raw_hex":"e829be3502","size":5,"soft_fail":false,"target_va":5417057760,"va":5379981234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94072,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94044,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":549,"record_offset":93768,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":91156,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":91072,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":549,"record_offset":91184,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":32143,"source_line":24,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::NormalCamera::rotation
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":87344,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":34157,"source_line":39,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":88260,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":34159,"source_line":221,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":91444,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87604,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":90896,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87516,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":88028,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87940,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":93980,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":91300,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":549,"record_offset":92340,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":87488,"parent_record_offset":87132,"ancestor_record_offset":87132,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":549,"record_offset":94264,"parent_record_offset":94240,"ancestor_record_offset":87132,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":549,"record_offset":94312,"parent_record_offset":94288,"ancestor_record_offset":87132,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":549,"record_offset":94288,"parent_record_offset":94264,"ancestor_record_offset":87132,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94240,"parent_record_offset":94216,"ancestor_record_offset":87132,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94216,"parent_record_offset":94192,"ancestor_record_offset":87132,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94164,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94096,"parent_record_offset":94072,"ancestor_record_offset":87132,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94952,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94192,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":94424,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":91492,"parent_record_offset":91468,"ancestor_record_offset":87132,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":91996,"parent_record_offset":91576,"ancestor_record_offset":87132,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":91904,"parent_record_offset":91848,"ancestor_record_offset":87132,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":91672,"parent_record_offset":91648,"ancestor_record_offset":87132,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":91704,"parent_record_offset":91620,"ancestor_record_offset":87132,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":91768,"parent_record_offset":91576,"ancestor_record_offset":87132,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94508,"parent_record_offset":94472,"ancestor_record_offset":87132,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":88360,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":87396,"parent_record_offset":87368,"ancestor_record_offset":87132,"inlinee":11830,"source_line":960,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":87760,"parent_record_offset":87736,"ancestor_record_offset":87132,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":549,"record_offset":94360,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":94388,"parent_record_offset":94360,"ancestor_record_offset":87132,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":549,"record_offset":93792,"parent_record_offset":93768,"ancestor_record_offset":87132,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":87544,"parent_record_offset":87516,"ancestor_record_offset":87132,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":87848,"parent_record_offset":87692,"ancestor_record_offset":87132,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":87792,"parent_record_offset":87692,"ancestor_record_offset":87132,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88440,"parent_record_offset":88416,"ancestor_record_offset":87132,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88056,"parent_record_offset":88028,"ancestor_record_offset":87132,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":94008,"parent_record_offset":93980,"ancestor_record_offset":87132,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":87968,"parent_record_offset":87940,"ancestor_record_offset":87132,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":93816,"parent_record_offset":93792,"ancestor_record_offset":87132,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":94688,"parent_record_offset":94096,"ancestor_record_offset":87132,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":549,"record_offset":91120,"parent_record_offset":91096,"ancestor_record_offset":87132,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":549,"record_offset":91096,"parent_record_offset":91072,"ancestor_record_offset":87132,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88660,"parent_record_offset":88616,"ancestor_record_offset":87132,"inlinee":32130,"source_line":170,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88928,"parent_record_offset":88616,"ancestor_record_offset":87132,"inlinee":32133,"source_line":175,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88616,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":32245,"source_line":149,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":549,"record_offset":87368,"parent_record_offset":87344,"ancestor_record_offset":87132,"inlinee":32143,"source_line":24,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":94472,"parent_record_offset":94448,"ancestor_record_offset":87132,"inlinee":34165,"source_line":212,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":89708,"parent_record_offset":88260,"ancestor_record_offset":87132,"inlinee":34161,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":88308,"parent_record_offset":88260,"ancestor_record_offset":87132,"inlinee":34160,"source_line":257,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::impl$1::update::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":94448,"parent_record_offset":94424,"ancestor_record_offset":87132,"inlinee":34163,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":87692,"parent_record_offset":87604,"ancestor_record_offset":87132,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":87452,"parent_record_offset":87368,"ancestor_record_offset":87132,"inlinee":32144,"source_line":194,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":91468,"parent_record_offset":91444,"ancestor_record_offset":87132,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":91576,"parent_record_offset":91468,"ancestor_record_offset":87132,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":92024,"parent_record_offset":91576,"ancestor_record_offset":87132,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":91540,"parent_record_offset":91516,"ancestor_record_offset":87132,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":91516,"parent_record_offset":91492,"ancestor_record_offset":87132,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88964,"parent_record_offset":88928,"ancestor_record_offset":87132,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88684,"parent_record_offset":88660,"ancestor_record_offset":87132,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88736,"parent_record_offset":88712,"ancestor_record_offset":87132,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88712,"parent_record_offset":88684,"ancestor_record_offset":87132,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88988,"parent_record_offset":88964,"ancestor_record_offset":87132,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88472,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89248,"parent_record_offset":88928,"ancestor_record_offset":87132,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88528,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88416,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89368,"parent_record_offset":88308,"ancestor_record_offset":87132,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87660,"parent_record_offset":87636,"ancestor_record_offset":87132,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87636,"parent_record_offset":87604,"ancestor_record_offset":87132,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89748,"parent_record_offset":89708,"ancestor_record_offset":87132,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89924,"parent_record_offset":89708,"ancestor_record_offset":87132,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":90712,"parent_record_offset":89708,"ancestor_record_offset":87132,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":91648,"parent_record_offset":91620,"ancestor_record_offset":87132,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":91620,"parent_record_offset":91576,"ancestor_record_offset":87132,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":87736,"parent_record_offset":87692,"ancestor_record_offset":87132,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x856D
// 首条位置: {"module":549,"inlinee":34157}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":549,"record_offset":87368,"parent_record_offset":87344,"ancestor_record_offset":87344,"inlinee":32143,"source_line":24,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":87396,"parent_record_offset":87368,"ancestor_record_offset":87344,"inlinee":11830,"source_line":960,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":87452,"parent_record_offset":87368,"ancestor_record_offset":87344,"inlinee":32144,"source_line":194,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":84852,"rva":11271104,"va":5379980224}
// 编译实例 RVAs: 0xabfbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141c12840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18164749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980224,"raw_hex":"e80d2c1501","size":5,"soft_fail":false,"target_va":5398145088,"va":5379980334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":85244,"parent_record_offset":84852,"ancestor_record_offset":84852,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":85216,"parent_record_offset":84852,"ancestor_record_offset":84852,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":85132,"parent_record_offset":84852,"ancestor_record_offset":84852,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":85104,"parent_record_offset":84852,"ancestor_record_offset":84852,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":85020,"parent_record_offset":84852,"ancestor_record_offset":84852,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":549,"record_offset":85180,"parent_record_offset":85156,"ancestor_record_offset":84852,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":549,"record_offset":85156,"parent_record_offset":85132,"ancestor_record_offset":84852,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":549,"record_offset":85068,"parent_record_offset":85044,"ancestor_record_offset":84852,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":549,"record_offset":85044,"parent_record_offset":85020,"ancestor_record_offset":84852,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":86736,"rva":11271744,"va":5379980864}
// 编译实例 RVAs: 0xabfe40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1773977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980864,"raw_hex":"e899111b00","size":5,"soft_fail":false,"target_va":5381754944,"va":5379980962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":87012,"parent_record_offset":86736,"ancestor_record_offset":86736,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":86908,"parent_record_offset":86736,"ancestor_record_offset":86736,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":87040,"parent_record_offset":86736,"ancestor_record_offset":86736,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":86936,"parent_record_offset":86908,"ancestor_record_offset":86736,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":87068,"parent_record_offset":87040,"ancestor_record_offset":86736,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":85276,"rva":11271376,"va":5379980496}
// 编译实例 RVAs: 0xabfcd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37077063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980496,"raw_hex":"e847c03502","size":5,"soft_fail":false,"target_va":5417057696,"va":5379980628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37077410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980496,"raw_hex":"e8a2c13502","size":5,"soft_fail":false,"target_va":5417058032,"va":5379980617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3432058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980496,"raw_hex":"e87a5e3400","size":5,"soft_fail":false,"target_va":5383412624,"va":5379980561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3431998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980496,"raw_hex":"e83e5e3400","size":5,"soft_fail":false,"target_va":5383412544,"va":5379980541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":85456,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":85484,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":11830,"source_line":960,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":85512,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":11736,"source_line":178,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":85660,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":86556,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":86644,"parent_record_offset":85276,"ancestor_record_offset":85276,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":85708,"parent_record_offset":85684,"ancestor_record_offset":85276,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":86212,"parent_record_offset":85792,"ancestor_record_offset":85276,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":86120,"parent_record_offset":86064,"ancestor_record_offset":85276,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":85888,"parent_record_offset":85864,"ancestor_record_offset":85276,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":85920,"parent_record_offset":85836,"ancestor_record_offset":85276,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":85984,"parent_record_offset":85792,"ancestor_record_offset":85276,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":86584,"parent_record_offset":86556,"ancestor_record_offset":85276,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":86672,"parent_record_offset":86644,"ancestor_record_offset":85276,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":85540,"parent_record_offset":85512,"ancestor_record_offset":85276,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":85628,"parent_record_offset":85512,"ancestor_record_offset":85276,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":85684,"parent_record_offset":85660,"ancestor_record_offset":85276,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":85792,"parent_record_offset":85684,"ancestor_record_offset":85276,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":86240,"parent_record_offset":85792,"ancestor_record_offset":85276,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":85756,"parent_record_offset":85732,"ancestor_record_offset":85276,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":85732,"parent_record_offset":85708,"ancestor_record_offset":85276,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":85864,"parent_record_offset":85836,"ancestor_record_offset":85276,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":85836,"parent_record_offset":85792,"ancestor_record_offset":85276,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":85588,"parent_record_offset":85564,"ancestor_record_offset":85276,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":85564,"parent_record_offset":85540,"ancestor_record_offset":85276,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8573
// 首条位置: {"module":549,"inlinee":34163}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":94472,"parent_record_offset":94448,"ancestor_record_offset":94448,"inlinee":34165,"source_line":212,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94508,"parent_record_offset":94472,"ancestor_record_offset":94448,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94620,"parent_record_offset":94472,"ancestor_record_offset":94448,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8575
// 首条位置: {"module":549,"inlinee":34165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94508,"parent_record_offset":94472,"ancestor_record_offset":94472,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":94620,"parent_record_offset":94472,"ancestor_record_offset":94472,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x856F
// 首条位置: {"module":549,"inlinee":34159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":89708,"parent_record_offset":88260,"ancestor_record_offset":88260,"inlinee":34161,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":549,"record_offset":88308,"parent_record_offset":88260,"ancestor_record_offset":88260,"inlinee":34160,"source_line":257,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":88360,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":88828,"parent_record_offset":88804,"ancestor_record_offset":88260,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":89272,"parent_record_offset":89248,"ancestor_record_offset":88260,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88496,"parent_record_offset":88472,"ancestor_record_offset":88260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88860,"parent_record_offset":88768,"ancestor_record_offset":88260,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88440,"parent_record_offset":88416,"ancestor_record_offset":88260,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88556,"parent_record_offset":88528,"ancestor_record_offset":88260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88660,"parent_record_offset":88616,"ancestor_record_offset":88260,"inlinee":32130,"source_line":170,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88928,"parent_record_offset":88616,"ancestor_record_offset":88260,"inlinee":32133,"source_line":175,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88616,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":32245,"source_line":149,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":88768,"parent_record_offset":88684,"ancestor_record_offset":88260,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88964,"parent_record_offset":88928,"ancestor_record_offset":88260,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88684,"parent_record_offset":88660,"ancestor_record_offset":88260,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88736,"parent_record_offset":88712,"ancestor_record_offset":88260,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88712,"parent_record_offset":88684,"ancestor_record_offset":88260,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88988,"parent_record_offset":88964,"ancestor_record_offset":88260,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88472,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89248,"parent_record_offset":88928,"ancestor_record_offset":88260,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88528,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88416,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89368,"parent_record_offset":88308,"ancestor_record_offset":88260,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89424,"parent_record_offset":89400,"ancestor_record_offset":88260,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89400,"parent_record_offset":89368,"ancestor_record_offset":88260,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89748,"parent_record_offset":89708,"ancestor_record_offset":88260,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89924,"parent_record_offset":89708,"ancestor_record_offset":88260,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":90712,"parent_record_offset":89708,"ancestor_record_offset":88260,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":88804,"parent_record_offset":88768,"ancestor_record_offset":88260,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_normal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8571
// 首条位置: {"module":549,"inlinee":34161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":90016,"parent_record_offset":89708,"ancestor_record_offset":89708,"inlinee":32245,"source_line":149,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89748,"parent_record_offset":89708,"ancestor_record_offset":89708,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89924,"parent_record_offset":89708,"ancestor_record_offset":89708,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":90712,"parent_record_offset":89708,"ancestor_record_offset":89708,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":90228,"parent_record_offset":90204,"ancestor_record_offset":89708,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":89956,"parent_record_offset":89924,"ancestor_record_offset":89708,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":89776,"parent_record_offset":89748,"ancestor_record_offset":89708,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":90260,"parent_record_offset":90168,"ancestor_record_offset":89708,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":90448,"parent_record_offset":90424,"ancestor_record_offset":89708,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":90060,"parent_record_offset":90016,"ancestor_record_offset":89708,"inlinee":32130,"source_line":170,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":90328,"parent_record_offset":90016,"ancestor_record_offset":89708,"inlinee":32133,"source_line":175,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":90168,"parent_record_offset":90084,"ancestor_record_offset":89708,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90364,"parent_record_offset":90328,"ancestor_record_offset":89708,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90084,"parent_record_offset":90060,"ancestor_record_offset":89708,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90136,"parent_record_offset":90112,"ancestor_record_offset":89708,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90112,"parent_record_offset":90084,"ancestor_record_offset":89708,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90388,"parent_record_offset":90364,"ancestor_record_offset":89708,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90592,"parent_record_offset":90328,"ancestor_record_offset":89708,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90648,"parent_record_offset":90328,"ancestor_record_offset":89708,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":90424,"parent_record_offset":90328,"ancestor_record_offset":89708,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":90204,"parent_record_offset":90168,"ancestor_record_offset":89708,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8570
// 首条位置: {"module":549,"inlinee":34160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":88360,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88616,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":32245,"source_line":149,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88472,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89312,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88528,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88416,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89368,"parent_record_offset":88308,"ancestor_record_offset":88308,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":88828,"parent_record_offset":88804,"ancestor_record_offset":88308,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":89272,"parent_record_offset":89248,"ancestor_record_offset":88308,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88496,"parent_record_offset":88472,"ancestor_record_offset":88308,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88860,"parent_record_offset":88768,"ancestor_record_offset":88308,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88440,"parent_record_offset":88416,"ancestor_record_offset":88308,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":88556,"parent_record_offset":88528,"ancestor_record_offset":88308,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88660,"parent_record_offset":88616,"ancestor_record_offset":88308,"inlinee":32130,"source_line":170,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":88928,"parent_record_offset":88616,"ancestor_record_offset":88308,"inlinee":32133,"source_line":175,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":88768,"parent_record_offset":88684,"ancestor_record_offset":88308,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88964,"parent_record_offset":88928,"ancestor_record_offset":88308,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88684,"parent_record_offset":88660,"ancestor_record_offset":88308,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88736,"parent_record_offset":88712,"ancestor_record_offset":88308,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88712,"parent_record_offset":88684,"ancestor_record_offset":88308,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":88988,"parent_record_offset":88964,"ancestor_record_offset":88308,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89192,"parent_record_offset":88928,"ancestor_record_offset":88308,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89248,"parent_record_offset":88928,"ancestor_record_offset":88308,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":89024,"parent_record_offset":88928,"ancestor_record_offset":88308,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89424,"parent_record_offset":89400,"ancestor_record_offset":88308,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":89400,"parent_record_offset":89368,"ancestor_record_offset":88308,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":88804,"parent_record_offset":88768,"ancestor_record_offset":88308,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
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
// 完整结构及原始类型记录: normal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::ActiveRotation
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "ref$", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "constructor": "core::option::IntoIter", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >", "constructor": "core::iter::traits::accum::impl$128::sum::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "f32", "f32", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0", "core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >,f32,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "f32", "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::NormalCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::normal::NormalCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::normal::NormalCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: normal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: normal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 信息 ID: 5e28d6cc93702df73616cbfc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=94472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=94764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 信息 ID: 8b384264184a601807f21e6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=87344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=88116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=94988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=95132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position
// 信息 ID: 2f089fadbaf6644e23902589
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=88260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_normal
// 信息 ID: d1c1ecce2baa74ad099568c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=89708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 信息 ID: daa27331a378d38b32b0c6e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=88308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 信息 ID: 94f030588bdfef8f55a8dba6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 信息 ID: e059358bbba27697e378c84d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 信息 ID: dbae649fb9588ac0c46ea8b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update
// 信息 ID: 0beeb9506cd9aa2660a83857
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 信息 ID: 747e690475c742554fb17e56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::camera_rig::normal::impl$1::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=94448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=549, inline_record=94740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
