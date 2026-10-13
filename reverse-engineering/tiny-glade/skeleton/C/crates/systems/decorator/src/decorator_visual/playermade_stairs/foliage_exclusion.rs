// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/foliage_exclusion.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c3613d13d6e17c0dc77138ed8c8706bcb600673779e56ac939c30a81df44cd2f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: foliage_exclusion.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 459, 1557, 1559, 1611, 1614
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B10
// 首条位置: {"module":459,"inlinee":27408}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":459,"record_offset":36756,"parent_record_offset":36724,"ancestor_record_offset":36724,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":37068,"parent_record_offset":37044,"ancestor_record_offset":36724,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":36892,"parent_record_offset":36864,"ancestor_record_offset":36724,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":36988,"parent_record_offset":36960,"ancestor_record_offset":36724,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":36960,"parent_record_offset":36924,"ancestor_record_offset":36724,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":37116,"parent_record_offset":37092,"ancestor_record_offset":36724,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":37092,"parent_record_offset":37068,"ancestor_record_offset":36724,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":459,"record_offset":36924,"parent_record_offset":36892,"ancestor_record_offset":36724,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":459,"record_offset":37044,"parent_record_offset":36820,"ancestor_record_offset":36724,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":459,"record_offset":36820,"parent_record_offset":36788,"ancestor_record_offset":36724,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":459,"record_offset":36864,"parent_record_offset":36820,"ancestor_record_offset":36724,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":459,"record_offset":37360,"parent_record_offset":37336,"ancestor_record_offset":36724,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":459,"record_offset":37616,"parent_record_offset":36820,"ancestor_record_offset":36724,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":459,"record_offset":37548,"parent_record_offset":37524,"ancestor_record_offset":36724,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":459,"record_offset":36788,"parent_record_offset":36756,"ancestor_record_offset":36724,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1611,"record_offset":3240,"parent_record_offset":3208,"ancestor_record_offset":3180,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":459,"record_offset":37180,"parent_record_offset":37156,"ancestor_record_offset":36724,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":459,"record_offset":37204,"parent_record_offset":37180,"ancestor_record_offset":36724,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":459,"record_offset":37228,"parent_record_offset":37204,"ancestor_record_offset":36724,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":459,"record_offset":37288,"parent_record_offset":37252,"ancestor_record_offset":36724,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":459,"record_offset":37156,"parent_record_offset":36820,"ancestor_record_offset":36724,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":459,"record_offset":37336,"parent_record_offset":37312,"ancestor_record_offset":36724,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":459,"record_offset":37252,"parent_record_offset":37228,"ancestor_record_offset":36724,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":459,"record_offset":37312,"parent_record_offset":37288,"ancestor_record_offset":36724,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":459,"record_offset":37428,"parent_record_offset":37252,"ancestor_record_offset":36724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":459,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":36724,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A10
// 首条位置: {"module":1557,"inlinee":80400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1614,"record_offset":2336,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":27398,"source_line":181,"archive":"_pdb/C/modules/01614.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1614,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":2304,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/01614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1611,"record_offset":272,"rva":21994480,"va":5390703600}
// 编译实例 RVAs: 0x14f9bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141501640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage_exclusion.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390703600,"raw_hex":"e8de780000","size":5,"soft_fail":false,"target_va":5390734912,"va":5390703965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage_exclusion.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25648919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390703600,"raw_hex":"e8175f8701","size":5,"soft_fail":false,"target_va":5416353040,"va":5390704116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1611,"record_offset":916,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_cols_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1611,"record_offset":468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80834,"source_line":73,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":748,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80836,"source_line":571,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/foliage_exclusion.rs
//     evidence={"module":1611,"record_offset":3180,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27408,"source_line":7,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":272,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":272,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3432,"parent_record_offset":3396,"ancestor_record_offset":272,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1611,"record_offset":3396,"parent_record_offset":3364,"ancestor_record_offset":272,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":3516,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":3336,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":272,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":272,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1611,"record_offset":4088,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1611,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":272,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1611,"record_offset":3268,"parent_record_offset":3208,"ancestor_record_offset":272,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1611,"record_offset":1064,"parent_record_offset":1028,"ancestor_record_offset":272,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1611,"record_offset":952,"parent_record_offset":916,"ancestor_record_offset":272,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1611,"record_offset":3240,"parent_record_offset":3208,"ancestor_record_offset":272,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1611,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":272,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1611,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":272,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1611,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":272,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1611,"record_offset":3760,"parent_record_offset":3724,"ancestor_record_offset":272,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1611,"record_offset":988,"parent_record_offset":952,"ancestor_record_offset":272,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1611,"record_offset":1028,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1611,"record_offset":1616,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1611,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":272,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":272,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1611,"record_offset":3628,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1611,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":272,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1611,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":272,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1611,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":272,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1611,"record_offset":1104,"parent_record_offset":1064,"ancestor_record_offset":272,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1611,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":272,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1611,"record_offset":3140,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1611,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":272,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":3900,"parent_record_offset":3724,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":272,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1611,"record_offset":1180,"parent_record_offset":1144,"ancestor_record_offset":272,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_cols_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1611,"record_offset":512,"parent_record_offset":468,"ancestor_record_offset":272,"inlinee":80835,"source_line":112,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1611,"record_offset":544,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":79472,"source_line":78,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::from_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":672,"parent_record_offset":468,"ancestor_record_offset":272,"inlinee":80837,"source_line":148,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":580,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":272,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1228,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1504,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1611,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":272,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad::closure$1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/foliage_exclusion.rs
//     evidence={"module":1611,"record_offset":1144,"parent_record_offset":1104,"ancestor_record_offset":272,"inlinee":80402,"source_line":28,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad::closure$0
// 记录形式: inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A0E
// 首条位置: {"module":1557,"inlinee":80398}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A12
// 首条位置: {"module":1557,"inlinee":80402}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1611,"record_offset":1180,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1611,"record_offset":1616,"parent_record_offset":1180,"ancestor_record_offset":1144,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1611,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":1144,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":1144,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":1320,"parent_record_offset":1228,"ancestor_record_offset":1144,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1611,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":1144,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1611,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1144,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1611,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1144,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1144,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":1144,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1228,"parent_record_offset":1180,"ancestor_record_offset":1144,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1611,"record_offset":1504,"parent_record_offset":1180,"ancestor_record_offset":1144,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01611.jsonl.gz"}
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
// 完整结构及原始类型记录: foliage_exclusion.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: foliage_exclusion.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: foliage_exclusion.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad
// 信息 ID: e7c2931f7ec90941fe68ac5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_stone_stair_steps; module=459, inline_record=36724; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad; module=1611, inline_record=3180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure$0
// 信息 ID: 4264bae0a40435861781c464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0 <- core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=2304; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=2468; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=2632; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=2796; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=2960; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=3132; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$24::map<usize,6,system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_foliage_quad::closure_env$0,glam::f32::sse2::vec4::Vec4>; module=1614, inline_record=3280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad
// 信息 ID: 653d7839342380abd1bb9955
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_ladder @ 0x89d2e1 -> 0x14f9bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::construct_wooden_stair_steps_impl @ 0x8ae0c9 -> 0x14f9bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad::closure$0
// 信息 ID: 0173f44ee63d9068da8c13f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad::closure$1
// 信息 ID: d7a70a644ff762a68ca2e2cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad; module=1611, inline_record=1144; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad; module=1611, inline_record=1836; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad; module=1611, inline_record=2320; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::foliage_exclusion::append_plank_foliage_quad; module=1611, inline_record=2756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
