// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/glade_themes/olden.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 7b80cd29c8a0c8d2c63a0cca4fe1523b269a0b908783b78c9f468c7deaf6f2ba
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: olden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 578
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::glade_themes::olden::startup_olden
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FC6
// 首条位置: {"module":578,"inlinee":36806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28080,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":578,"record_offset":27636,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":29668,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":30352,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":28016,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":578,"record_offset":30424,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":578,"record_offset":28308,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":28807,"source_line":75,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::preplaced_autoclutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/preplaced_autoclutter.rs
//     evidence={"module":578,"record_offset":27608,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":36808,"source_line":5,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":27876,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":8070,"source_line":291,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":29124,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":20184,"source_line":1357,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29420,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":578,"record_offset":27988,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":578,"record_offset":28524,"parent_record_offset":27372,"ancestor_record_offset":27372,"inlinee":27093,"source_line":24,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":29876,"parent_record_offset":29852,"ancestor_record_offset":27372,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":27372,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":29924,"parent_record_offset":29900,"ancestor_record_offset":27372,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":29852,"parent_record_offset":29764,"ancestor_record_offset":27372,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28184,"parent_record_offset":28152,"ancestor_record_offset":27372,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":29792,"parent_record_offset":29764,"ancestor_record_offset":27372,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":29764,"parent_record_offset":29740,"ancestor_record_offset":27372,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28232,"parent_record_offset":28208,"ancestor_record_offset":27372,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":27372,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":29740,"parent_record_offset":29716,"ancestor_record_offset":27372,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28152,"parent_record_offset":28108,"ancestor_record_offset":27372,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":28108,"parent_record_offset":28080,"ancestor_record_offset":27372,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":578,"record_offset":27660,"parent_record_offset":27636,"ancestor_record_offset":27372,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":578,"record_offset":27716,"parent_record_offset":27684,"ancestor_record_offset":27372,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":578,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":27372,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":29208,"parent_record_offset":29152,"ancestor_record_offset":27372,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":28804,"parent_record_offset":28724,"ancestor_record_offset":27372,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":578,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":27372,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":29564,"parent_record_offset":29540,"ancestor_record_offset":27372,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":29820,"parent_record_offset":29792,"ancestor_record_offset":27372,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":578,"record_offset":28484,"parent_record_offset":28460,"ancestor_record_offset":27372,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":578,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":27372,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":27800,"parent_record_offset":27740,"ancestor_record_offset":27372,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":29692,"parent_record_offset":29668,"ancestor_record_offset":27372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":28044,"parent_record_offset":28016,"ancestor_record_offset":27372,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":27372,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":28272,"parent_record_offset":28108,"ancestor_record_offset":27372,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":578,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":27372,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":27772,"parent_record_offset":27740,"ancestor_record_offset":27372,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":27924,"parent_record_offset":27900,"ancestor_record_offset":27372,"inlinee":27998,"source_line":52,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":578,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":27372,"inlinee":27997,"source_line":208,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":578,"record_offset":28372,"parent_record_offset":28344,"ancestor_record_offset":27372,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":28552,"parent_record_offset":28524,"ancestor_record_offset":27372,"inlinee":27094,"source_line":226,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":578,"record_offset":28588,"parent_record_offset":28552,"ancestor_record_offset":27372,"inlinee":27095,"source_line":192,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":578,"record_offset":28344,"parent_record_offset":28308,"ancestor_record_offset":27372,"inlinee":26854,"source_line":146,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":28636,"parent_record_offset":28588,"ancestor_record_offset":27372,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":29060,"parent_record_offset":28552,"ancestor_record_offset":27372,"inlinee":27101,"source_line":1962,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$19::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":29152,"parent_record_offset":29124,"ancestor_record_offset":27372,"inlinee":36810,"source_line":1280,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":578,"record_offset":28724,"parent_record_offset":28552,"ancestor_record_offset":27372,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29508,"parent_record_offset":29476,"ancestor_record_offset":27372,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29476,"parent_record_offset":29444,"ancestor_record_offset":27372,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":29444,"parent_record_offset":29420,"ancestor_record_offset":27372,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":578,"record_offset":28460,"parent_record_offset":28344,"ancestor_record_offset":27372,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":27372,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":28396,"parent_record_offset":28372,"ancestor_record_offset":27372,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":29540,"parent_record_offset":29508,"ancestor_record_offset":27372,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":578,"record_offset":27952,"parent_record_offset":27900,"ancestor_record_offset":27372,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00578.jsonl.gz"}
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
// 完整结构及原始类型记录: olden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: olden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: olden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::startup::glade_themes::olden::startup_olden
// 信息 ID: c8cc20ccc5da051d6269f204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::clearing::init_clearing; module=578, inline_record=27372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
