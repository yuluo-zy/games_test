// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7a76af24153b78f40d4216613f436bf15689e20cbe3a33e553e88c1a964d2252
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1517, 3068
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 首条位置: {"module":1517,"record_offset":1704,"rva":19366576,"va":5388075696}
// 编译实例 RVAs: 0x12782b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12995486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e862b439ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388076969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12995627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e8d5b339ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388077126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10766962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e88eb55bff","size":5,"soft_fail":false,"target_va":5377309600,"va":5388076557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7678347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e875d68aff","size":5,"soft_fail":false,"target_va":5380398224,"va":5388076566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-112285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e86349feff","size":5,"soft_fail":false,"target_va":5387964304,"va":5388076584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23462903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e8f7036601","size":5,"soft_fail":false,"target_va":5411540992,"va":5388078084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5236725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e80b18b0ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5388076928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23464082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e892086601","size":5,"soft_fail":false,"target_va":5411542064,"va":5388077977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23463830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e896076601","size":5,"soft_fail":false,"target_va":5411541904,"va":5388078069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10334022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e8ba5062ff","size":5,"soft_fail":false,"target_va":5377743024,"va":5388077041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7649433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075696,"raw_hex":"e867478bff","size":5,"soft_fail":false,"target_va":5380427664,"va":5388077092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1517,"record_offset":3792,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1517,"record_offset":3904,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1517,"record_offset":3876,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1517,"record_offset":2896,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":4120,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":2980,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1517,"record_offset":3352,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1517,"record_offset":1960,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1517,"record_offset":4608,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1517,"record_offset":3268,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$39::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1517,"record_offset":3736,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":71505,"source_line":1354,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":4664,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":2616,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":2652,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":2748,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":1704,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":1704,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":1704,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":1704,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":3992,"parent_record_offset":3960,"ancestor_record_offset":1704,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":4272,"parent_record_offset":4216,"ancestor_record_offset":1704,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1517,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":1704,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1517,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":1704,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":4244,"parent_record_offset":4216,"ancestor_record_offset":1704,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":1704,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":1704,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1517,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":1704,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":2400,"parent_record_offset":1984,"ancestor_record_offset":1704,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":2308,"parent_record_offset":2252,"ancestor_record_offset":1704,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1704,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":1704,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":2108,"parent_record_offset":2028,"ancestor_record_offset":1704,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1517,"record_offset":2172,"parent_record_offset":1984,"ancestor_record_offset":1704,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1517,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":1704,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1517,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1704,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1517,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":1704,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1517,"record_offset":2684,"parent_record_offset":2652,"ancestor_record_offset":1704,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1517,"record_offset":2780,"parent_record_offset":2748,"ancestor_record_offset":1704,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":1704,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":1704,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":1704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":1704,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":3060,"parent_record_offset":3008,"ancestor_record_offset":1704,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":1704,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":1704,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1517,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1704,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1517,"record_offset":2428,"parent_record_offset":1984,"ancestor_record_offset":1704,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":4756,"parent_record_offset":4688,"ancestor_record_offset":1704,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1517,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":1704,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1517,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1704,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1517,"record_offset":2028,"parent_record_offset":1984,"ancestor_record_offset":1704,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::xxx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1517,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":1704,"inlinee":71502,"source_line":100,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1517,"record_offset":3476,"parent_record_offset":3352,"ancestor_record_offset":1704,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::zzz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1517,"record_offset":3604,"parent_record_offset":3352,"ancestor_record_offset":1704,"inlinee":71503,"source_line":250,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1517,"record_offset":4792,"parent_record_offset":4756,"ancestor_record_offset":1704,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1517,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":1704,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1517,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1704,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01517.jsonl.gz"}
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
// 首条位置: {"module":1517,"record_offset":6544,"rva":19369680,"va":5388078800}
// 编译实例 RVAs: 0x1278ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12997378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e8feac39ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388078861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12997725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e8a3ab39ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388079224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23461834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e8caff6501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388079153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23461783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e897ff6501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388079175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10334985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e8f74c62ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388078996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10334885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e85b4d62ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388079088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e82c4a62ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388078975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e82a4c62ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388079057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388078800,"raw_hex":"e8b24c62ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388079017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":6768,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7452,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1517,"record_offset":6916,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":7536,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":6544,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":6544,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":6544,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":6800,"parent_record_offset":6768,"ancestor_record_offset":6544,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":6544,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1517,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":6544,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":6544,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":7236,"parent_record_offset":7204,"ancestor_record_offset":6544,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":6544,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":6544,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":6544,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7084,"parent_record_offset":7056,"ancestor_record_offset":6544,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":6544,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7056,"parent_record_offset":7028,"ancestor_record_offset":6544,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":6544,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1517,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":6544,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1517,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":6544,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":6544,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":7148,"parent_record_offset":7112,"ancestor_record_offset":6544,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1517,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":6544,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1517,"record_offset":7364,"parent_record_offset":7112,"ancestor_record_offset":6544,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":6544,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01517.jsonl.gz"}
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
// 首条位置: {"module":1517,"record_offset":7652,"rva":19370128,"va":5388079248}
// 编译实例 RVAs: 0x1279090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12997826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e83eab39ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388079309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12998173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e8e3a939ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388079672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23461386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e80afe6501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388079601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23461335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e8d7fd6501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388079623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e8374b62ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388079444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e89b4b62ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388079536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10336148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e86c4862ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388079423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e86a4a62ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388079505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079248,"raw_hex":"e8f24a62ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388079465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7876,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8560,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1517,"record_offset":8024,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":8644,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":7652,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":7652,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":7652,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":7652,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":7652,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1517,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":7652,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":8284,"parent_record_offset":8256,"ancestor_record_offset":7652,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":8344,"parent_record_offset":8312,"ancestor_record_offset":7652,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":8312,"parent_record_offset":8284,"ancestor_record_offset":7652,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8220,"parent_record_offset":8192,"ancestor_record_offset":7652,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":7652,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8192,"parent_record_offset":8164,"ancestor_record_offset":7652,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":7652,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8164,"parent_record_offset":8136,"ancestor_record_offset":7652,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":7652,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1517,"record_offset":8080,"parent_record_offset":8052,"ancestor_record_offset":7652,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1517,"record_offset":8052,"parent_record_offset":8024,"ancestor_record_offset":7652,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":7652,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":8256,"parent_record_offset":8220,"ancestor_record_offset":7652,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1517,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":7652,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1517,"record_offset":8472,"parent_record_offset":8220,"ancestor_record_offset":7652,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7652,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01517.jsonl.gz"}
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
// 首条位置: {"module":1517,"record_offset":8760,"rva":19370576,"va":5388079696}
// 编译实例 RVAs: 0x1279250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12998274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e87ea939ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388079757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12998621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e823a839ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388080120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23460938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e84afc6501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388080049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23460887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e817fc6501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388080071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e8774962ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388079892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e8db4962ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388079984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10336596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e8ac4662ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388079871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10336086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e8aa4862ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388079953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10335950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388079696,"raw_hex":"e8324962ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388079913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":8984,"parent_record_offset":8760,"ancestor_record_offset":8760,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9668,"parent_record_offset":8760,"ancestor_record_offset":8760,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1517,"record_offset":9132,"parent_record_offset":8760,"ancestor_record_offset":8760,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":9752,"parent_record_offset":8760,"ancestor_record_offset":8760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":8760,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":8760,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":8760,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":9016,"parent_record_offset":8984,"ancestor_record_offset":8760,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":8760,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1517,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":8760,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":9392,"parent_record_offset":9364,"ancestor_record_offset":8760,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":9452,"parent_record_offset":9420,"ancestor_record_offset":8760,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":9420,"parent_record_offset":9392,"ancestor_record_offset":8760,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9328,"parent_record_offset":9300,"ancestor_record_offset":8760,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":8760,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9300,"parent_record_offset":9272,"ancestor_record_offset":8760,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9244,"parent_record_offset":9216,"ancestor_record_offset":8760,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9272,"parent_record_offset":9244,"ancestor_record_offset":8760,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1517,"record_offset":9216,"parent_record_offset":9188,"ancestor_record_offset":8760,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1517,"record_offset":9188,"parent_record_offset":9160,"ancestor_record_offset":8760,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1517,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":8760,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":8760,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":9364,"parent_record_offset":9328,"ancestor_record_offset":8760,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1517,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":8760,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1517,"record_offset":9580,"parent_record_offset":9328,"ancestor_record_offset":8760,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":8760,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01517.jsonl.gz"}
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
// 首条位置: {"module":1517,"record_offset":9868,"rva":19371024,"va":5388080144}
// 编译实例 RVAs: 0x1279410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb70
//     owner=unresolved_namespace; count=2
//     target_aliases=211 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19127652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e89c22dcfe","size":5,"soft_fail":false,"target_va":5368953712,"va":5388081359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13001832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e8989b39ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388083331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10773493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e80b9c5bff","size":5,"soft_fail":false,"target_va":5377307216,"va":5388080704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7682512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e830c68aff","size":5,"soft_fail":false,"target_va":5380398224,"va":5388080731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412592f0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-135232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e8c0effdff","size":5,"soft_fail":false,"target_va":5387948784,"va":5388084011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e80a39feff","size":5,"soft_fail":false,"target_va":5387964304,"va":5388080769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f82f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23587396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e844ea6701","size":5,"soft_fail":false,"target_va":5411668720,"va":5388081319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7447250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e82e5d8eff","size":5,"soft_fail":false,"target_va":5380634736,"va":5388081981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23458306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e802f26501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388083465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23458611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e833f36501","size":5,"soft_fail":false,"target_va":5411542064,"va":5388083448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23458319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e80ff26501","size":5,"soft_fail":false,"target_va":5411541904,"va":5388083580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10772860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e8849e5bff","size":5,"soft_fail":false,"target_va":5377307552,"va":5388080407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10773237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e80b9d5bff","size":5,"soft_fail":false,"target_va":5377309152,"va":5388082384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8768691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e84d337aff","size":5,"soft_fail":false,"target_va":5379311616,"va":5388080302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7679149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e853d38aff","size":5,"soft_fail":false,"target_va":5380401520,"va":5388080664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7683825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e80fc18aff","size":5,"soft_fail":false,"target_va":5380396720,"va":5388080540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e8aed1ffff","size":5,"soft_fail":false,"target_va":5388070448,"va":5388082301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388080144,"raw_hex":"e8ef080000","size":5,"soft_fail":false,"target_va":5388084352,"va":5388082060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":12608,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":12740,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1517,"record_offset":10816,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1517,"record_offset":10332,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10172,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":13296,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10460,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10272,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10212,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":13168,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10136,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":13380,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":10492,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1517,"record_offset":12188,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":29552,"source_line":1182,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1517,"record_offset":11660,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1517,"record_offset":12276,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":19787,"source_line":1232,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1517,"record_offset":10744,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":10244,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":10136,"source_line":779,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":10400,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":11441,"source_line":1114,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12636,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12364,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":35795,"source_line":2290,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":9868,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1517,"record_offset":13728,"parent_record_offset":13644,"ancestor_record_offset":9868,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":9868,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":9868,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":9868,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":9868,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":9868,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1517,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":9868,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1517,"record_offset":12772,"parent_record_offset":12740,"ancestor_record_offset":9868,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1517,"record_offset":10892,"parent_record_offset":10864,"ancestor_record_offset":9868,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1517,"record_offset":11008,"parent_record_offset":10916,"ancestor_record_offset":9868,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1517,"record_offset":10616,"parent_record_offset":10592,"ancestor_record_offset":9868,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1517,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":9868,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1517,"record_offset":11128,"parent_record_offset":10864,"ancestor_record_offset":9868,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1517,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":9868,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1517,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":9868,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1517,"record_offset":13548,"parent_record_offset":13520,"ancestor_record_offset":9868,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1517,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":9868,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1517,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":9868,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1517,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":9868,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1517,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":9868,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1517,"record_offset":12216,"parent_record_offset":12188,"ancestor_record_offset":9868,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1517,"record_offset":10776,"parent_record_offset":10744,"ancestor_record_offset":9868,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1517,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":9868,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1517,"record_offset":10952,"parent_record_offset":10916,"ancestor_record_offset":9868,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1517,"record_offset":10916,"parent_record_offset":10892,"ancestor_record_offset":9868,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":9868,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":9868,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1517,"record_offset":12948,"parent_record_offset":12916,"ancestor_record_offset":9868,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1517,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":9868,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":13496,"parent_record_offset":13380,"ancestor_record_offset":9868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1517,"record_offset":11592,"parent_record_offset":11472,"ancestor_record_offset":9868,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":9868,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1517,"record_offset":10428,"parent_record_offset":10400,"ancestor_record_offset":9868,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":10652,"parent_record_offset":10532,"ancestor_record_offset":9868,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":9868,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1517,"record_offset":10592,"parent_record_offset":10560,"ancestor_record_offset":9868,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12672,"parent_record_offset":12636,"ancestor_record_offset":9868,"inlinee":48744,"source_line":1716,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12420,"parent_record_offset":12364,"ancestor_record_offset":9868,"inlinee":11444,"source_line":952,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12576,"parent_record_offset":12364,"ancestor_record_offset":9868,"inlinee":35796,"source_line":919,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":11560,"parent_record_offset":11528,"ancestor_record_offset":9868,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":12468,"parent_record_offset":12420,"ancestor_record_offset":9868,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":11528,"parent_record_offset":11472,"ancestor_record_offset":9868,"inlinee":11432,"source_line":984,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":11628,"parent_record_offset":11472,"ancestor_record_offset":9868,"inlinee":6214,"source_line":659,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":13520,"parent_record_offset":13496,"ancestor_record_offset":9868,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":13408,"parent_record_offset":13380,"ancestor_record_offset":9868,"inlinee":43484,"source_line":2228,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1517,"record_offset":13456,"parent_record_offset":13432,"ancestor_record_offset":9868,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":1517,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":9868,"inlinee":71507,"source_line":197,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":1517,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":9868,"inlinee":71508,"source_line":214,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":1517,"record_offset":13200,"parent_record_offset":13168,"ancestor_record_offset":9868,"inlinee":71509,"source_line":337,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1517,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":9868,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1517,"record_offset":10704,"parent_record_offset":10652,"ancestor_record_offset":9868,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11753
// 首条位置: {"module":1517,"inlinee":71507}
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
// IPI IDs: 0x11754
// 首条位置: {"module":1517,"inlinee":71508}
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
// 首条位置: {"module":1517,"record_offset":14428,"rva":19375232,"va":5388084352}
// 编译实例 RVAs: 0x127a480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19366425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8e77dd8fe","size":5,"soft_fail":false,"target_va":5368718192,"va":5388084612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=211 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19131128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e80815dcfe","size":5,"soft_fail":false,"target_va":5368953712,"va":5388084835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6194950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8fa78a1ff","size":5,"soft_fail":false,"target_va":5381889600,"va":5388084545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23456812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e82cec6501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388084959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8779929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e867077aff","size":5,"soft_fail":false,"target_va":5379304864,"va":5388084788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8774208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8c01d7aff","size":5,"soft_fail":false,"target_va":5379310224,"va":5388084427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7666928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e810038bff","size":5,"soft_fail":false,"target_va":5380417680,"va":5388084603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7667183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e811028bff","size":5,"soft_fail":false,"target_va":5380417344,"va":5388084522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8291831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8097a81ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5388084450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::get_tangent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6193422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8f27ea1ff","size":5,"soft_fail":false,"target_va":5381891312,"va":5388084729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6194470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8da7aa1ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5388084657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6198853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388084352,"raw_hex":"e8bb69a1ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5388084752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":14920,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":14640,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1517,"record_offset":14748,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1517,"record_offset":14892,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1517,"record_offset":14780,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/01517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11755
// 首条位置: {"module":1517,"inlinee":71509}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id
// 信息 ID: 0d696376c45e36ffd94bab40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a566c -> 0x12782b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$0
// 信息 ID: 88dfc2970161854cdcf60348
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1278262 -> 0x1278ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$1
// 信息 ID: 85af6f10dbcbfe3ad7faeee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x12782a2 -> 0x1279090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$2
// 信息 ID: 0094ec77f0272cd290796ac1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1278282 -> 0x1279250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part
// 信息 ID: cb8298dfdbfbe39edc27abf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a60f8 -> 0x1279410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 信息 ID: a4dfe4fcfc4923667c24356a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=1517, inline_record=10300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
// 信息 ID: 8ee33f708b286f5e71accf77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=1517, inline_record=11176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
// 信息 ID: 532539a0334305b02d70bdb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1279b8c -> 0x127a480 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1279d44 -> 0x127a480 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1279ef6 -> 0x127a480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$5
// 信息 ID: 1e84e95871c51be18f2a06ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=1517, inline_record=13200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
