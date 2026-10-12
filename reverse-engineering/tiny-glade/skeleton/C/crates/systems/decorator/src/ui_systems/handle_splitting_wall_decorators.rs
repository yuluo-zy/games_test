// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 7a76af24153b78f40d4216613f436bf15689e20cbe3a33e553e88c1a964d2252
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 473, 1574
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
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":5412,"rva":21836352,"va":5390545472}
// 编译实例 RVAs: 0x14d3240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14868028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8c4211dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390546711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14868165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e83b211dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390546864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21824509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e803fcb2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390546696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fbce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12417279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8018742ff","size":5,"soft_fail":false,"target_va":5378129120,"va":5390546394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-205849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8e7dbfcff","size":5,"soft_fail":false,"target_va":5390340576,"va":5390546420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26073153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e841d88d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390547611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5751816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8f83ba8ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5390546675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26074565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8c5dd8d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390547638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26074513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e891dd8d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390547722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8895544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e8c84378ff","size":5,"soft_fail":false,"target_va":5381650864,"va":5390546403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11611041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e85fd44eff","size":5,"soft_fail":false,"target_va":5378935744,"va":5390546780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10334866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390545472,"raw_hex":"e86e4d62ff","size":5,"soft_fail":false,"target_va":5380211968,"va":5390546829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1574,"record_offset":7536,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1574,"record_offset":7648,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1574,"record_offset":7620,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1574,"record_offset":6604,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":7872,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":6688,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1574,"record_offset":7096,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":79479,"source_line":707,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1574,"record_offset":5668,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1574,"record_offset":8484,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1574,"record_offset":6984,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$39::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1574,"record_offset":7480,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":80503,"source_line":1608,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":8596,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":6324,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":6360,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":6456,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":5412,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":5412,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":5412,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":5412,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":5412,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":5412,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":7744,"parent_record_offset":7712,"ancestor_record_offset":5412,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":8024,"parent_record_offset":7968,"ancestor_record_offset":5412,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":7712,"parent_record_offset":7676,"ancestor_record_offset":5412,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":7676,"parent_record_offset":7648,"ancestor_record_offset":5412,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1574,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":5412,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":7996,"parent_record_offset":7968,"ancestor_record_offset":5412,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":5412,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":5412,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1574,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":5412,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1574,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":5412,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":6112,"parent_record_offset":5692,"ancestor_record_offset":5412,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":6020,"parent_record_offset":5964,"ancestor_record_offset":5412,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5412,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":5412,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":5820,"parent_record_offset":5736,"ancestor_record_offset":5412,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1574,"record_offset":5884,"parent_record_offset":5692,"ancestor_record_offset":5412,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1574,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":5412,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1574,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":5412,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1574,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":5412,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1574,"record_offset":6392,"parent_record_offset":6360,"ancestor_record_offset":5412,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1574,"record_offset":6488,"parent_record_offset":6456,"ancestor_record_offset":5412,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":5412,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":5412,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":5412,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":5412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":5412,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":6772,"parent_record_offset":6716,"ancestor_record_offset":5412,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":6716,"parent_record_offset":6688,"ancestor_record_offset":5412,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":5412,"inlinee":27106,"source_line":244,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1574,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":5412,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1574,"record_offset":6140,"parent_record_offset":5692,"ancestor_record_offset":5412,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":8688,"parent_record_offset":8620,"ancestor_record_offset":5412,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1574,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":5412,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1574,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":5412,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1574,"record_offset":5736,"parent_record_offset":5692,"ancestor_record_offset":5412,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::xxx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1574,"record_offset":7124,"parent_record_offset":7096,"ancestor_record_offset":5412,"inlinee":79665,"source_line":120,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1574,"record_offset":7220,"parent_record_offset":7096,"ancestor_record_offset":5412,"inlinee":79483,"source_line":180,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::zzz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1574,"record_offset":7348,"parent_record_offset":7096,"ancestor_record_offset":5412,"inlinee":79666,"source_line":245,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1574,"record_offset":8724,"parent_record_offset":8688,"ancestor_record_offset":5412,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1574,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":5412,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1574,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":5412,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":11908,"rva":21839728,"va":5390548848}
// 编译实例 RVAs: 0x14d3f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14870224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e830191dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390548907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14870573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e8d3171dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390549272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21826705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e86ff3b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390548892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e80ad28d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390549202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e813d28d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390549218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e89acd4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390549041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11610405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e8dbd64eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390549136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e800ce4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390549019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e84bf64eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390549104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e814f64eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390549063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":12844,"parent_record_offset":11908,"ancestor_record_offset":11908,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":12132,"parent_record_offset":11908,"ancestor_record_offset":11908,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1574,"record_offset":12312,"parent_record_offset":11908,"ancestor_record_offset":11908,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":12928,"parent_record_offset":11908,"ancestor_record_offset":11908,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":11908,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":11908,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":11908,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":11908,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":11908,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":11908,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":12192,"parent_record_offset":12160,"ancestor_record_offset":11908,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":12976,"parent_record_offset":12952,"ancestor_record_offset":11908,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":12160,"parent_record_offset":12132,"ancestor_record_offset":11908,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":11908,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":12544,"parent_record_offset":12516,"ancestor_record_offset":11908,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":12604,"parent_record_offset":12572,"ancestor_record_offset":11908,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":12572,"parent_record_offset":12544,"ancestor_record_offset":11908,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":11908,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":12424,"parent_record_offset":12396,"ancestor_record_offset":11908,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":11908,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":11908,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":12396,"parent_record_offset":12368,"ancestor_record_offset":11908,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1574,"record_offset":12368,"parent_record_offset":12340,"ancestor_record_offset":11908,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1574,"record_offset":12340,"parent_record_offset":12312,"ancestor_record_offset":11908,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":12892,"parent_record_offset":12868,"ancestor_record_offset":11908,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":12516,"parent_record_offset":12480,"ancestor_record_offset":11908,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":12760,"parent_record_offset":12480,"ancestor_record_offset":11908,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":12784,"parent_record_offset":12760,"ancestor_record_offset":11908,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":10716,"rva":21839280,"va":5390548400}
// 编译实例 RVAs: 0x14d3db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14869776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8f01a1dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390548459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14870125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e893191dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390548824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21826257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e82ff5b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390548444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26072010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8cad38d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390548754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26072019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8d3d38d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390548770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e85acf4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390548593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11609957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e89bd84eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390548688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8c0cf4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390548571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11601909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e80bf84eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390548656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11601964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8d4f74eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390548615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":11652,"parent_record_offset":10716,"ancestor_record_offset":10716,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":10940,"parent_record_offset":10716,"ancestor_record_offset":10716,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1574,"record_offset":11120,"parent_record_offset":10716,"ancestor_record_offset":10716,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":11736,"parent_record_offset":10716,"ancestor_record_offset":10716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":10716,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10716,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":10716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":10716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10716,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":10716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11000,"parent_record_offset":10968,"ancestor_record_offset":10716,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":10716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10716,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":10716,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":11352,"parent_record_offset":11324,"ancestor_record_offset":10716,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":11412,"parent_record_offset":11380,"ancestor_record_offset":10716,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":11380,"parent_record_offset":11352,"ancestor_record_offset":10716,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":11676,"parent_record_offset":11652,"ancestor_record_offset":10716,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":11232,"parent_record_offset":11204,"ancestor_record_offset":10716,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":11260,"parent_record_offset":11232,"ancestor_record_offset":10716,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":11288,"parent_record_offset":11260,"ancestor_record_offset":10716,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":11204,"parent_record_offset":11176,"ancestor_record_offset":10716,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1574,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":10716,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1574,"record_offset":11148,"parent_record_offset":11120,"ancestor_record_offset":10716,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":11700,"parent_record_offset":11676,"ancestor_record_offset":10716,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":11324,"parent_record_offset":11288,"ancestor_record_offset":10716,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":11568,"parent_record_offset":11288,"ancestor_record_offset":10716,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":10716,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":13100,"rva":21840176,"va":5390549296}
// 编译实例 RVAs: 0x14d4130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14870672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e870171dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390549355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14871021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e813161dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390549720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21827153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e8aff1b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390549340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e84ad08d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390549650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e853d08d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390549666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11613222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e8dacb4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390549489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11610853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e81bd54eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390549584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11613120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e840cc4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390549467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e88bf44eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390549552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e854f44eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390549511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":14036,"parent_record_offset":13100,"ancestor_record_offset":13100,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":13324,"parent_record_offset":13100,"ancestor_record_offset":13100,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1574,"record_offset":13504,"parent_record_offset":13100,"ancestor_record_offset":13100,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":14120,"parent_record_offset":13100,"ancestor_record_offset":13100,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":13100,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":13100,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":13100,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":13100,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13456,"parent_record_offset":13432,"ancestor_record_offset":13100,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":13100,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":13384,"parent_record_offset":13352,"ancestor_record_offset":13100,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":13100,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":13352,"parent_record_offset":13324,"ancestor_record_offset":13100,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1574,"record_offset":14144,"parent_record_offset":14120,"ancestor_record_offset":13100,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":13100,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":13796,"parent_record_offset":13764,"ancestor_record_offset":13100,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":13100,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":13100,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":13616,"parent_record_offset":13588,"ancestor_record_offset":13100,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":13644,"parent_record_offset":13616,"ancestor_record_offset":13100,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":13100,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1574,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":13100,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1574,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":13100,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1574,"record_offset":13532,"parent_record_offset":13504,"ancestor_record_offset":13100,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":13100,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":13708,"parent_record_offset":13672,"ancestor_record_offset":13100,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":13952,"parent_record_offset":13672,"ancestor_record_offset":13100,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13100,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":272,"rva":21831744,"va":5390540864}
// 编译实例 RVAs: 0x14d2040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14865288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8782c1dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390543987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ae60
//     owner=unresolved_namespace; count=2
//     target_aliases=234 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21329599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e84189bafe","size":5,"soft_fail":false,"target_va":5369212512,"va":5390542106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12414473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8f79142ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390541412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-200912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e830effcff","size":5,"soft_fail":false,"target_va":5390340576,"va":5390541483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9670
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-169966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e81268fdff","size":5,"soft_fail":false,"target_va":5390374512,"va":5390544473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd0f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26208833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e841ea8f01","size":5,"soft_fail":false,"target_va":5416750912,"va":5390542074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25839161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e839468a01","size":5,"soft_fail":false,"target_va":5416381792,"va":5390542626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26077519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e84fe98d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390544076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26078053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e865eb8d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390544150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26078018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e842eb8d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390544217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12414184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8189342ff","size":5,"soft_fail":false,"target_va":5378126928,"va":5390541107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12415006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8e28f42ff","size":5,"soft_fail":false,"target_va":5378128064,"va":5390543065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11049619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e86d6557ff","size":5,"soft_fail":false,"target_va":5379491392,"va":5390541006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8890582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e82a5778ff","size":5,"soft_fail":false,"target_va":5381650864,"va":5390541441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8887051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8f56478ff","size":5,"soft_fail":false,"target_va":5381654320,"va":5390541366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10324997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8fb7362ff","size":5,"soft_fail":false,"target_va":5380216256,"va":5390541248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8a3e3ffff","size":5,"soft_fail":false,"target_va":5390535728,"va":5390542984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390540864,"raw_hex":"e8fc070000","size":5,"soft_fail":false,"target_va":5390544768,"va":5390542719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":2856,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":2984,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1574,"record_offset":1168,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1574,"record_offset":736,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":576,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":3560,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":864,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":676,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":616,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":3468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":540,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":3644,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":896,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1574,"record_offset":2468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":30716,"source_line":1293,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1574,"record_offset":2020,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1574,"record_offset":2524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":21757,"source_line":1343,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1574,"record_offset":1096,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":648,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26280,"source_line":779,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":804,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26281,"source_line":1114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2884,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2612,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1574,"record_offset":3992,"parent_record_offset":3908,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1574,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":3040,"parent_record_offset":3008,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1574,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1574,"record_offset":1224,"parent_record_offset":1196,"ancestor_record_offset":272,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1574,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":272,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1574,"record_offset":1380,"parent_record_offset":1280,"ancestor_record_offset":272,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1574,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":272,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1574,"record_offset":1528,"parent_record_offset":1380,"ancestor_record_offset":272,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1574,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":272,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1574,"record_offset":3812,"parent_record_offset":3784,"ancestor_record_offset":272,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1574,"record_offset":1348,"parent_record_offset":1320,"ancestor_record_offset":272,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1574,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1574,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1574,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":272,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1574,"record_offset":1128,"parent_record_offset":1096,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1574,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":272,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1574,"record_offset":1320,"parent_record_offset":1280,"ancestor_record_offset":272,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1574,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":272,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":968,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":3220,"parent_record_offset":3188,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1574,"record_offset":1468,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":3760,"parent_record_offset":3644,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1574,"record_offset":1952,"parent_record_offset":1832,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":272,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1574,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1574,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1574,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":1032,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":272,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1574,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":272,"inlinee":27106,"source_line":244,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2920,"parent_record_offset":2884,"ancestor_record_offset":272,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2668,"parent_record_offset":2612,"ancestor_record_offset":272,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2824,"parent_record_offset":2612,"ancestor_record_offset":272,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":272,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":2716,"parent_record_offset":2668,"ancestor_record_offset":272,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":1888,"parent_record_offset":1832,"ancestor_record_offset":272,"inlinee":15353,"source_line":984,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":1988,"parent_record_offset":1832,"ancestor_record_offset":272,"inlinee":4829,"source_line":659,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":272,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":3672,"parent_record_offset":3644,"ancestor_record_offset":272,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1574,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":272,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":1574,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":272,"inlinee":80501,"source_line":197,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":1574,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":27641,"source_line":214,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1574,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":272,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A75
// 首条位置: {"module":1574,"inlinee":80501}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BF9
// 首条位置: {"module":473,"inlinee":27641}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1574,"record_offset":4860,"rva":21835648,"va":5390544768}
// 编译实例 RVAs: 0x14d2f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ae60
//     owner=unresolved_namespace; count=1
//     target_aliases=234 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21332764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e8e47cbafe","size":5,"soft_fail":false,"target_va":5369212512,"va":5390545271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20770691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e87d10c3fe","size":5,"soft_fail":false,"target_va":5369774336,"va":5390545022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de62e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7261542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e89a3291ff","size":5,"soft_fail":false,"target_va":5383283424,"va":5390544961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26076200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e828e48d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390545395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8890029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e8535978ff","size":5,"soft_fail":false,"target_va":5381655200,"va":5390545224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11035360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e8209d57ff","size":5,"soft_fail":false,"target_va":5379509488,"va":5390544843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9312506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e806e771ff","size":5,"soft_fail":false,"target_va":5381232512,"va":5390545013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9313199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e851e471ff","size":5,"soft_fail":false,"target_va":5381231744,"va":5390544938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9641335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e889e26cff","size":5,"soft_fail":false,"target_va":5380903536,"va":5390544866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::get_tangent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7263171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e83d2c91ff","size":5,"soft_fail":false,"target_va":5383281984,"va":5390545150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7262854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e87a2d91ff","size":5,"soft_fail":false,"target_va":5383282224,"va":5390545073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7269466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390544768,"raw_hex":"e8a61391ff","size":5,"soft_fail":false,"target_va":5383275712,"va":5390545173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":5348,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":5072,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1574,"record_offset":5180,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1574,"record_offset":5320,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01574.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1574,"record_offset":5212,"parent_record_offset":4860,"ancestor_record_offset":4860,"inlinee":27690,"source_line":213,"archive":"_pdb/C/modules/01574.jsonl.gz"}
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
// 完整结构及原始类型记录: handle_splitting_wall_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_splitting_wall_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_splitting_wall_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id
// 信息 ID: fac651ef12a8be6a290a75f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x8614cf -> 0x14d3240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$0
// 信息 ID: 94794fa90efbd54ed6d4f325
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d4310 -> 0x14d3f70 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$1
// 信息 ID: cf6dc92b4d7eac7515564e3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d42f0 -> 0x14d3db0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$2
// 信息 ID: 15eef6f66d103770593bedc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d4300 -> 0x14d4130 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part
// 信息 ID: 31bceda349a98fc240d763ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x861d7c -> 0x14d2040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 信息 ID: 77c18416262b5f42d5de9904
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=1574, inline_record=704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
// 信息 ID: e86fb2f28b3399a8addf6c10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=1574, inline_record=1576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
// 信息 ID: 09e5af04c886369aa4a7408b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x14d277f -> 0x14d2f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x14d2942 -> 0x14d2f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x14d2ac7 -> 0x14d2f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
