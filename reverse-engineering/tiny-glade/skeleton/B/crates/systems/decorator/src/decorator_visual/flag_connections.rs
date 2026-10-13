// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/flag_connections.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: dcb34fc08640dac9e1d258a82c84cc27b24a51998c7b3629dff3dce8b8ae4aeb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flag_connections.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 438, 1495, 3079, 4073, 4083, 4084
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::generate_flag_connections
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":2044,"rva":8254576,"va":5376963696}
// 编译实例 RVAs: 0x7df470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2290097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e84f0eddff","size":5,"soft_fail":false,"target_va":5374676688,"va":5376966780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1885885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e84339e3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376967384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140839340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t365775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8cf940500","size":5,"soft_fail":false,"target_va":5377332032,"va":5376966252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2345279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e83fc92300","size":5,"soft_fail":false,"target_va":5379310240,"va":5376964956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3437351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e827733400","size":5,"soft_fail":false,"target_va":5380401776,"va":5376964420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141258f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10977162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e88a7fa700","size":5,"soft_fail":false,"target_va":5387947824,"va":5376970657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-341207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e829cbfaff","size":5,"soft_fail":false,"target_va":5376625152,"va":5376966354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34571896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e878860f02","size":5,"soft_fail":false,"target_va":5411541584,"va":5376969683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34572590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e82e890f02","size":5,"soft_fail":false,"target_va":5411542296,"va":5376969701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e832fbffff","size":5,"soft_fail":false,"target_va":5376962640,"va":5376963865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34579420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8dca30f02","size":5,"soft_fail":false,"target_va":5411543360,"va":5376963935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2338546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8f2ae2300","size":5,"soft_fail":false,"target_va":5379303968,"va":5376965417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2354695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e807ee2300","size":5,"soft_fail":false,"target_va":5379319216,"va":5376964516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2343938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e802c42300","size":5,"soft_fail":false,"target_va":5379310144,"va":5376966201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3436075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e82b6e3400","size":5,"soft_fail":false,"target_va":5380401520,"va":5376965440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4421155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e823764300","size":5,"soft_fail":false,"target_va":5381385536,"va":5376964376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4426344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8688a4300","size":5,"soft_fail":false,"target_va":5381390640,"va":5376964291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4417041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e811664300","size":5,"soft_fail":false,"target_va":5381386480,"va":5376969434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4420863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8ff744300","size":5,"soft_fail":false,"target_va":5381384704,"va":5376963836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34675174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8e6191102","size":5,"soft_fail":false,"target_va":5411644800,"va":5376969621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10972707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8236ea700","size":5,"soft_fail":false,"target_va":5387942848,"va":5376970136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34567428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e804750f02","size":5,"soft_fail":false,"target_va":5411537104,"va":5376969671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::flag_connections::clothline
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e8ef1b0000","size":5,"soft_fail":false,"target_va":5376976144,"va":5376968988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::flag_connections::greedily_connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376963696,"raw_hex":"e89f110000","size":5,"soft_fail":false,"target_va":5376971824,"va":5376967308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":4776,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":10992,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":12784,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":438,"record_offset":12568,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":438,"record_offset":11920,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4555,"source_line":56,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":438,"record_offset":11948,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":4860,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":438,"record_offset":4832,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":12332,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":438,"record_offset":12624,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":12708,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":438,"record_offset":4720,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":13096,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":8448,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13681,"source_line":910,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":7676,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":14788,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":14344,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13651,"source_line":1297,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":14492,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":8360,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":3736,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":2304,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":2760,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":438,"record_offset":6276,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":438,"record_offset":11248,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":438,"record_offset":6636,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24932,"source_line":334,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":2044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":12496,"parent_record_offset":12404,"ancestor_record_offset":2044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":11128,"parent_record_offset":11096,"ancestor_record_offset":2044,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":12432,"parent_record_offset":12404,"ancestor_record_offset":2044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":12404,"parent_record_offset":12380,"ancestor_record_offset":2044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":2044,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":2044,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":2044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":11096,"parent_record_offset":11064,"ancestor_record_offset":2044,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":438,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":2044,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":438,"record_offset":12148,"parent_record_offset":11972,"ancestor_record_offset":2044,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":438,"record_offset":15808,"parent_record_offset":15780,"ancestor_record_offset":2044,"inlinee":16077,"source_line":352,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4072,"parent_record_offset":4044,"ancestor_record_offset":2044,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2044,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":2044,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":2044,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2044,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4216,"parent_record_offset":4044,"ancestor_record_offset":2044,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":2044,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":2044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":438,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":2044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":438,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":438,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":2044,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":438,"record_offset":5576,"parent_record_offset":5540,"ancestor_record_offset":2044,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":8144,"parent_record_offset":7724,"ancestor_record_offset":2044,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":8052,"parent_record_offset":7996,"ancestor_record_offset":2044,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":2044,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":7852,"parent_record_offset":7768,"ancestor_record_offset":2044,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":7916,"parent_record_offset":7724,"ancestor_record_offset":2044,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":438,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":2044,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":438,"record_offset":7412,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":2044,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":2044,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":2044,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":438,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":2044,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":438,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":2044,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":438,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":2044,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":438,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":2044,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":438,"record_offset":12464,"parent_record_offset":12432,"ancestor_record_offset":2044,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":438,"record_offset":2460,"parent_record_offset":2428,"ancestor_record_offset":2044,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":438,"record_offset":5376,"parent_record_offset":5344,"ancestor_record_offset":2044,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":438,"record_offset":5344,"parent_record_offset":5312,"ancestor_record_offset":2044,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":14568,"parent_record_offset":14492,"ancestor_record_offset":2044,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":14404,"parent_record_offset":14344,"ancestor_record_offset":2044,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":2044,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":14968,"parent_record_offset":14844,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":438,"record_offset":3888,"parent_record_offset":3844,"ancestor_record_offset":2044,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":438,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":2044,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":438,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":2044,"inlinee":11824,"source_line":46,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":438,"record_offset":12092,"parent_record_offset":11972,"ancestor_record_offset":2044,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":438,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":2044,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":438,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":2044,"inlinee":13189,"source_line":163,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":2044,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":5972,"parent_record_offset":4984,"ancestor_record_offset":2044,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":5852,"parent_record_offset":5064,"ancestor_record_offset":2044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":6032,"parent_record_offset":4960,"ancestor_record_offset":2044,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":4676,"parent_record_offset":3844,"ancestor_record_offset":2044,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":12260,"parent_record_offset":12172,"ancestor_record_offset":2044,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":4280,"parent_record_offset":3992,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":438,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":2044,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":438,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":2044,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2044,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2044,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":3356,"parent_record_offset":3328,"ancestor_record_offset":2044,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":12284,"parent_record_offset":12260,"ancestor_record_offset":2044,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":2044,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":438,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":2044,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":12228,"parent_record_offset":12204,"ancestor_record_offset":2044,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":12812,"parent_record_offset":12784,"ancestor_record_offset":2044,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":12204,"parent_record_offset":12172,"ancestor_record_offset":2044,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":2044,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":438,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":2044,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2044,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2044,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":3076,"parent_record_offset":2960,"ancestor_record_offset":2044,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":438,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":2044,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":438,"record_offset":2788,"parent_record_offset":2760,"ancestor_record_offset":2044,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":438,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":2044,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":438,"record_offset":2896,"parent_record_offset":2860,"ancestor_record_offset":2044,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2044,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2044,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2044,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":3400,"parent_record_offset":2860,"ancestor_record_offset":2044,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2400,"parent_record_offset":2368,"ancestor_record_offset":2044,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":2044,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2560,"parent_record_offset":2528,"ancestor_record_offset":2044,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2656,"parent_record_offset":2560,"ancestor_record_offset":2044,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":2044,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2340,"parent_record_offset":2304,"ancestor_record_offset":2044,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":2044,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":2960,"parent_record_offset":2928,"ancestor_record_offset":2044,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":3244,"parent_record_offset":2960,"ancestor_record_offset":2044,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":3216,"parent_record_offset":2960,"ancestor_record_offset":2044,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":3020,"parent_record_offset":2960,"ancestor_record_offset":2044,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":438,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":2044,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":438,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":2044,"inlinee":24786,"source_line":746,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":2044,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":2044,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":2044,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":438,"record_offset":8172,"parent_record_offset":7724,"ancestor_record_offset":2044,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":15028,"parent_record_offset":14844,"ancestor_record_offset":2044,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":14844,"parent_record_offset":14788,"ancestor_record_offset":2044,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":438,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":2044,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":438,"record_offset":7768,"parent_record_offset":7724,"ancestor_record_offset":2044,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":5272,"parent_record_offset":5232,"ancestor_record_offset":2044,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":5312,"parent_record_offset":5272,"ancestor_record_offset":2044,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":5420,"parent_record_offset":5232,"ancestor_record_offset":2044,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":5232,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":6888,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":5456,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":7440,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":2044,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":5512,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":6972,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":438,"record_offset":7592,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":438,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":2044,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":6700,"parent_record_offset":6668,"ancestor_record_offset":2044,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":6732,"parent_record_offset":6700,"ancestor_record_offset":2044,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2044,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":7296,"parent_record_offset":7188,"ancestor_record_offset":2044,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":2044,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":7632,"parent_record_offset":6732,"ancestor_record_offset":2044,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":2044,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":5760,"parent_record_offset":5116,"ancestor_record_offset":2044,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":5612,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":7532,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6432,"parent_record_offset":6372,"ancestor_record_offset":2044,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":5184,"parent_record_offset":5148,"ancestor_record_offset":2044,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6800,"parent_record_offset":6772,"ancestor_record_offset":2044,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6772,"parent_record_offset":6732,"ancestor_record_offset":2044,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":2044,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6404,"parent_record_offset":6372,"ancestor_record_offset":2044,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":2044,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":2044,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":7564,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":5148,"parent_record_offset":5116,"ancestor_record_offset":2044,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":7188,"parent_record_offset":6828,"ancestor_record_offset":2044,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":3772,"parent_record_offset":3736,"ancestor_record_offset":2044,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":16196,"parent_record_offset":16164,"ancestor_record_offset":2044,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":16712,"parent_record_offset":16452,"ancestor_record_offset":2044,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":438,"record_offset":3916,"parent_record_offset":3888,"ancestor_record_offset":2044,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":3992,"parent_record_offset":3844,"ancestor_record_offset":2044,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":4252,"parent_record_offset":3992,"ancestor_record_offset":2044,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":438,"record_offset":6668,"parent_record_offset":6636,"ancestor_record_offset":2044,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":438,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":2044,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":438,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":2044,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":15188,"parent_record_offset":15028,"ancestor_record_offset":2044,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":438,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":2044,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":438,"record_offset":3844,"parent_record_offset":3808,"ancestor_record_offset":2044,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":438,"record_offset":3808,"parent_record_offset":3772,"ancestor_record_offset":2044,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect
// 记录形式: compiled_procedure; instances=1
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":18296,"rva":8262704,"va":5376971824}
// 编译实例 RVAs: 0x7e1430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7372499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e82d818fff","size":5,"soft_fail":false,"target_va":5369600800,"va":5376973294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6915511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8497a96ff","size":5,"soft_fail":false,"target_va":5370059888,"va":5376975394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1892975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8911de3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376974474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1891891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8cd21e3ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5376973422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412569b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10964314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e85a4da700","size":5,"soft_fail":false,"target_va":5387938224,"va":5376973905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125a080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10976186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8ba7ba700","size":5,"soft_fail":false,"target_va":5387952256,"va":5376976065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125b750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10983230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e83e97a700","size":5,"soft_fail":false,"target_va":5387958096,"va":5376974861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125b890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10985160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8c89ea700","size":5,"soft_fail":false,"target_va":5387958416,"va":5376973251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125ea20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10998511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8efd2a700","size":5,"soft_fail":false,"target_va":5387971104,"va":5376972588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141267af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11033828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8e45ca800","size":5,"soft_fail":false,"target_va":5388008176,"va":5376974343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2618116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8fc0cd8ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5376973103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34566045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e89d6f0f02","size":5,"soft_fail":false,"target_va":5411540963,"va":5376974913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34566701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e82d720f02","size":5,"soft_fail":false,"target_va":5411541584,"va":5376974878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34567480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e838750f02","size":5,"soft_fail":false,"target_va":5411542296,"va":5376974811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11209550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e84e0bab00","size":5,"soft_fail":false,"target_va":5388182816,"va":5376973261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e887dbffff","size":5,"soft_fail":false,"target_va":5376962640,"va":5376971972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34571313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e831840f02","size":5,"soft_fail":false,"target_va":5411543360,"va":5376972042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::intersects_any
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t964623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e80fb80e00","size":5,"soft_fail":false,"target_va":5377938864,"va":5376974236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4413045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e875564300","size":5,"soft_fail":false,"target_va":5381385536,"va":5376972486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4418234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8ba6a4300","size":5,"soft_fail":false,"target_va":5381390640,"va":5376972401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4411832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8b8514300","size":5,"soft_fail":false,"target_va":5381386480,"va":5376974643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4412762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e85a554300","size":5,"soft_fail":false,"target_va":5381384704,"va":5376971937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34670053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8e5051102","size":5,"soft_fail":false,"target_va":5411644800,"va":5376974742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10967610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e83a5aa700","size":5,"soft_fail":false,"target_va":5387942848,"va":5376975233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34562303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376971824,"raw_hex":"e8ff600f02","size":5,"soft_fail":false,"target_va":5411537104,"va":5376974796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":438,"record_offset":20968,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":21140,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":21440,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":23108,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":24808,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":22152,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":25512,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":25484,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":25184,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":438,"record_offset":21252,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":438,"record_offset":21632,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":25784,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":438,"record_offset":21168,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":438,"record_offset":22272,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":24748,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":22356,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":25000,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":21964,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":25348,"source_line":484,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":25300,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":25392,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":25212,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":19984,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":18560,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":19008,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":438,"record_offset":23700,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":438,"record_offset":25728,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":438,"record_offset":22620,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":22124,"parent_record_offset":18296,"ancestor_record_offset":18296,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":18296,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":18296,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":26140,"parent_record_offset":26112,"ancestor_record_offset":18296,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":23328,"parent_record_offset":23236,"ancestor_record_offset":18296,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":438,"record_offset":26112,"parent_record_offset":25960,"ancestor_record_offset":18296,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":21512,"parent_record_offset":21480,"ancestor_record_offset":18296,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":23172,"parent_record_offset":23140,"ancestor_record_offset":18296,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":26388,"parent_record_offset":26356,"ancestor_record_offset":18296,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":26356,"parent_record_offset":26328,"ancestor_record_offset":18296,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":18296,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":18296,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":23236,"parent_record_offset":23204,"ancestor_record_offset":18296,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":23204,"parent_record_offset":23172,"ancestor_record_offset":18296,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":26328,"parent_record_offset":26300,"ancestor_record_offset":18296,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":438,"record_offset":23272,"parent_record_offset":23236,"ancestor_record_offset":18296,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":438,"record_offset":21000,"parent_record_offset":20968,"ancestor_record_offset":18296,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":21480,"parent_record_offset":21440,"ancestor_record_offset":18296,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":438,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":18296,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":438,"record_offset":23140,"parent_record_offset":23108,"ancestor_record_offset":18296,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":438,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":18296,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20320,"parent_record_offset":20292,"ancestor_record_offset":18296,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":22904,"parent_record_offset":22876,"ancestor_record_offset":18296,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":18296,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20348,"parent_record_offset":20320,"ancestor_record_offset":18296,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20268,"parent_record_offset":20240,"ancestor_record_offset":18296,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":18296,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20464,"parent_record_offset":20292,"ancestor_record_offset":18296,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":438,"record_offset":20576,"parent_record_offset":20552,"ancestor_record_offset":18296,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":24436,"parent_record_offset":24412,"ancestor_record_offset":18296,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":24388,"parent_record_offset":24364,"ancestor_record_offset":18296,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":438,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":18296,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":438,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":18296,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":438,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":18296,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":438,"record_offset":21072,"parent_record_offset":21032,"ancestor_record_offset":18296,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":438,"record_offset":21664,"parent_record_offset":21632,"ancestor_record_offset":18296,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":438,"record_offset":21692,"parent_record_offset":21632,"ancestor_record_offset":18296,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":438,"record_offset":21776,"parent_record_offset":21632,"ancestor_record_offset":18296,"inlinee":25349,"source_line":60,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":438,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":18296,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":438,"record_offset":26048,"parent_record_offset":26024,"ancestor_record_offset":18296,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":438,"record_offset":26424,"parent_record_offset":26388,"ancestor_record_offset":18296,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":438,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":18296,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":438,"record_offset":24188,"parent_record_offset":24160,"ancestor_record_offset":18296,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":438,"record_offset":24160,"parent_record_offset":24132,"ancestor_record_offset":18296,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":25424,"parent_record_offset":25392,"ancestor_record_offset":18296,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":22028,"parent_record_offset":21988,"ancestor_record_offset":18296,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":438,"record_offset":20136,"parent_record_offset":20092,"ancestor_record_offset":18296,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":21928,"parent_record_offset":21776,"ancestor_record_offset":18296,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":18956,"parent_record_offset":18928,"ancestor_record_offset":18296,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":20924,"parent_record_offset":20092,"ancestor_record_offset":18296,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":438,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":18296,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":20528,"parent_record_offset":20240,"ancestor_record_offset":18296,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":438,"record_offset":21216,"parent_record_offset":21192,"ancestor_record_offset":18296,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":438,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":18296,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":438,"record_offset":21864,"parent_record_offset":21840,"ancestor_record_offset":18296,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":21396,"parent_record_offset":21372,"ancestor_record_offset":18296,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":19420,"parent_record_offset":19396,"ancestor_record_offset":18296,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":18296,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":21840,"parent_record_offset":21804,"ancestor_record_offset":18296,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":18296,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":20420,"parent_record_offset":20396,"ancestor_record_offset":18296,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":24836,"parent_record_offset":24808,"ancestor_record_offset":18296,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":22188,"parent_record_offset":22152,"ancestor_record_offset":18296,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":24864,"parent_record_offset":24836,"ancestor_record_offset":18296,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":438,"record_offset":22224,"parent_record_offset":22188,"ancestor_record_offset":18296,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":438,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":18296,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":19348,"parent_record_offset":19324,"ancestor_record_offset":18296,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":18296,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":19372,"parent_record_offset":19348,"ancestor_record_offset":18296,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":438,"record_offset":22296,"parent_record_offset":22272,"ancestor_record_offset":18296,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":19676,"parent_record_offset":19648,"ancestor_record_offset":18296,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":21284,"parent_record_offset":21252,"ancestor_record_offset":18296,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":21804,"parent_record_offset":21776,"ancestor_record_offset":18296,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":19324,"parent_record_offset":19208,"ancestor_record_offset":18296,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":21348,"parent_record_offset":21284,"ancestor_record_offset":18296,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":438,"record_offset":21032,"parent_record_offset":21000,"ancestor_record_offset":18296,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":438,"record_offset":18676,"parent_record_offset":18648,"ancestor_record_offset":18296,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":438,"record_offset":19036,"parent_record_offset":19008,"ancestor_record_offset":18296,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":438,"record_offset":19176,"parent_record_offset":19144,"ancestor_record_offset":18296,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":438,"record_offset":19144,"parent_record_offset":19108,"ancestor_record_offset":18296,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":18296,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":18296,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":19060,"parent_record_offset":19036,"ancestor_record_offset":18296,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":438,"record_offset":19648,"parent_record_offset":19108,"ancestor_record_offset":18296,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18648,"parent_record_offset":18616,"ancestor_record_offset":18296,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":18296,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18808,"parent_record_offset":18776,"ancestor_record_offset":18296,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18904,"parent_record_offset":18808,"ancestor_record_offset":18296,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18776,"parent_record_offset":18752,"ancestor_record_offset":18296,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18592,"parent_record_offset":18560,"ancestor_record_offset":18296,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":438,"record_offset":18928,"parent_record_offset":18904,"ancestor_record_offset":18296,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":19208,"parent_record_offset":19176,"ancestor_record_offset":18296,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":19492,"parent_record_offset":19208,"ancestor_record_offset":18296,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":19464,"parent_record_offset":19208,"ancestor_record_offset":18296,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":438,"record_offset":19268,"parent_record_offset":19208,"ancestor_record_offset":18296,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":438,"record_offset":24412,"parent_record_offset":24388,"ancestor_record_offset":18296,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":25112,"parent_record_offset":25024,"ancestor_record_offset":18296,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":25024,"parent_record_offset":25000,"ancestor_record_offset":18296,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":22056,"parent_record_offset":21988,"ancestor_record_offset":18296,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":18296,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":24100,"parent_record_offset":24072,"ancestor_record_offset":18296,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":24132,"parent_record_offset":24100,"ancestor_record_offset":18296,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":438,"record_offset":24072,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":23932,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":24512,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":438,"record_offset":24016,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":438,"record_offset":24664,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":438,"record_offset":24364,"parent_record_offset":24340,"ancestor_record_offset":18296,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":23756,"parent_record_offset":23728,"ancestor_record_offset":18296,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":18296,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":23064,"parent_record_offset":22704,"ancestor_record_offset":18296,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":24340,"parent_record_offset":24232,"ancestor_record_offset":18296,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":22704,"parent_record_offset":22676,"ancestor_record_offset":18296,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":438,"record_offset":24704,"parent_record_offset":23784,"ancestor_record_offset":18296,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":24280,"parent_record_offset":24256,"ancestor_record_offset":18296,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":24256,"parent_record_offset":24232,"ancestor_record_offset":18296,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":24604,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":23848,"parent_record_offset":23820,"ancestor_record_offset":18296,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":23820,"parent_record_offset":23784,"ancestor_record_offset":18296,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":25992,"parent_record_offset":25960,"ancestor_record_offset":18296,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":25904,"parent_record_offset":25880,"ancestor_record_offset":18296,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":23876,"parent_record_offset":23848,"ancestor_record_offset":18296,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":25960,"parent_record_offset":25904,"ancestor_record_offset":18296,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":25932,"parent_record_offset":25904,"ancestor_record_offset":18296,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":24636,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":26024,"parent_record_offset":25992,"ancestor_record_offset":18296,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":18296,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":438,"record_offset":24232,"parent_record_offset":23876,"ancestor_record_offset":18296,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":438,"record_offset":23728,"parent_record_offset":23700,"ancestor_record_offset":18296,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":438,"record_offset":25752,"parent_record_offset":25728,"ancestor_record_offset":18296,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":438,"record_offset":22676,"parent_record_offset":22648,"ancestor_record_offset":18296,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":20020,"parent_record_offset":19984,"ancestor_record_offset":18296,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":26808,"parent_record_offset":26776,"ancestor_record_offset":18296,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":27324,"parent_record_offset":27064,"ancestor_record_offset":18296,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":438,"record_offset":20164,"parent_record_offset":20136,"ancestor_record_offset":18296,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":20240,"parent_record_offset":20092,"ancestor_record_offset":18296,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":438,"record_offset":20500,"parent_record_offset":20240,"ancestor_record_offset":18296,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":438,"record_offset":22648,"parent_record_offset":22620,"ancestor_record_offset":18296,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":25144,"parent_record_offset":25112,"ancestor_record_offset":18296,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":438,"record_offset":22760,"parent_record_offset":22732,"ancestor_record_offset":18296,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":438,"record_offset":22876,"parent_record_offset":22820,"ancestor_record_offset":18296,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":438,"record_offset":22732,"parent_record_offset":22704,"ancestor_record_offset":18296,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":438,"record_offset":20192,"parent_record_offset":20164,"ancestor_record_offset":18296,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":438,"record_offset":20092,"parent_record_offset":20056,"ancestor_record_offset":18296,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":438,"record_offset":20056,"parent_record_offset":20020,"ancestor_record_offset":18296,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x116C6
// 首条位置: {"module":1495,"inlinee":71366}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$32::cmp
//     owner=private_source_ownership_candidate; count=28
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1495,"record_offset":20776,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":71349,"source_line":155,"archive":"_pdb/B/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::cmp
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1495,"record_offset":20800,"parent_record_offset":20776,"ancestor_record_offset":20752,"inlinee":10268,"source_line":1836,"archive":"_pdb/B/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=45
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x116C4
// 首条位置: {"module":1495,"inlinee":71364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1495,"record_offset":624,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::clothline
// 记录形式: compiled_procedure; instances=1
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":28384,"rva":8267024,"va":5376976144}
// 编译实例 RVAs: 0x7e2510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412ab270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11307140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376976144,"raw_hex":"e88488ac00","size":5,"soft_fail":false,"target_va":5388284528,"va":5376977383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34871373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376976144,"raw_hex":"e84d181402","size":5,"soft_fail":false,"target_va":5411847776,"va":5376976398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":28804,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":28832,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":438,"record_offset":29420,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":31224,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30656,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":28592,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":28864,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30400,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30104,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30160,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":438,"record_offset":29176,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":28776,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::flag_connections::clothline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag_connections.rs
//     evidence={"module":438,"record_offset":29296,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":25351,"source_line":425,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":438,"record_offset":29356,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":25101,"source_line":99,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":438,"record_offset":29392,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":25352,"source_line":158,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":438,"record_offset":31648,"parent_record_offset":28384,"ancestor_record_offset":28384,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":438,"record_offset":29596,"parent_record_offset":29528,"ancestor_record_offset":28384,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":29712,"parent_record_offset":29688,"ancestor_record_offset":28384,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":438,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":28384,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":438,"record_offset":29528,"parent_record_offset":29500,"ancestor_record_offset":28384,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":438,"record_offset":29500,"parent_record_offset":29456,"ancestor_record_offset":28384,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":438,"record_offset":29624,"parent_record_offset":29528,"ancestor_record_offset":28384,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":438,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":28384,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":438,"record_offset":29648,"parent_record_offset":29624,"ancestor_record_offset":28384,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":29112,"parent_record_offset":29080,"ancestor_record_offset":28384,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":28952,"parent_record_offset":28908,"ancestor_record_offset":28384,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":28648,"parent_record_offset":28616,"ancestor_record_offset":28384,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":438,"record_offset":29564,"parent_record_offset":29528,"ancestor_record_offset":28384,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":438,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":28384,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":438,"record_offset":31352,"parent_record_offset":31256,"ancestor_record_offset":28384,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":31256,"parent_record_offset":31224,"ancestor_record_offset":28384,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":28704,"parent_record_offset":28616,"ancestor_record_offset":28384,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30460,"parent_record_offset":30424,"ancestor_record_offset":28384,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":30424,"parent_record_offset":30400,"ancestor_record_offset":28384,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":28908,"parent_record_offset":28864,"ancestor_record_offset":28384,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":29080,"parent_record_offset":28864,"ancestor_record_offset":28384,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":28616,"parent_record_offset":28592,"ancestor_record_offset":28384,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":438,"record_offset":29688,"parent_record_offset":29456,"ancestor_record_offset":28384,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":438,"record_offset":29456,"parent_record_offset":29420,"ancestor_record_offset":28384,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":29324,"parent_record_offset":29296,"ancestor_record_offset":28384,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":28736,"parent_record_offset":28704,"ancestor_record_offset":28384,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::clothline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6307
// 首条位置: {"module":438,"inlinee":25351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":29324,"parent_record_offset":29296,"ancestor_record_offset":29296,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00438.jsonl.gz"}
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
// 完整结构及原始类型记录: flag_connections.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::flag_connections::CandidateInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>", "alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::vec::impl$20::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>", "system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flag_connections.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flag_connections.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::clothline
// 信息 ID: d657bb4e2e4be4bc44d12f89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x7e091c -> 0x7e2510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x7e09fe -> 0x7e2510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e712c -> 0x7e2510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::clothline::closure$0
// 信息 ID: 7d477aa62850338912213155
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::flag_connections::clothline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=438, inline_record=29296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::generate_flag_connections
// 信息 ID: 706fa32e5e080f815c51df92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e0f99e25d3a57682c5c2e7cd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x37451f -> 0x7df470 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)> > > @ 0x45a509 -> 0x7df470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect
// 信息 ID: 2b6c940adf08126945eebd2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x7e028c -> 0x7e1430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e572e -> 0x7e1430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$0
// 信息 ID: 7c2ed1a4dc29dfd3edb4b3f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::flag_connections::greedily_connect", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::stable::driftsort_main<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::vec::Vec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::stable::drift::sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::shared::pivot::median3_rec<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=20752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=20948; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=23824; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=24028; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=24172; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=24328; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=24528; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=1495, inline_record=25016; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$1
// 信息 ID: d82cd1b199cef80dbf1dec59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::flag_connections::greedily_connect", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::stable::driftsort_main<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::vec::Vec<tuple$<usize,f32>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::stable::drift::sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=604; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=704; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=868; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=988; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=1176; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=1512; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=5340; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=1495, inline_record=5756; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
