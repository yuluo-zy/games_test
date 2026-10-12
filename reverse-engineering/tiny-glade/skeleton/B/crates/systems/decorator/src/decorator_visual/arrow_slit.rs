// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/arrow_slit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6cf90892319dda7d481045d9c1cfbef261a8a87b20e5fc280804f39ac5345daf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: arrow_slit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 429, 435
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::arrow_slit::generate_arrow_slits
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":429,"record_offset":1924,"rva":8109792,"va":5376818912}
// 编译实例 RVAs: 0x7bbee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7553676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e874bd8cff","size":5,"soft_fail":false,"target_va":5369267664,"va":5376821335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2144253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e80348dfff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376820584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2143593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8974adfff","size":5,"soft_fail":false,"target_va":5374676688,"va":5376820276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t486428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e81c6c0700","size":5,"soft_fail":false,"target_va":5377307216,"va":5376820783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408393b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t508322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8a2c10700","size":5,"soft_fail":false,"target_va":5377332144,"va":5376823817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1047820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e80cfd0f00","size":5,"soft_fail":false,"target_va":5377869088,"va":5376821263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2485885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e87dee2500","size":5,"soft_fail":false,"target_va":5379305728,"va":5376819838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3577419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e84b963600","size":5,"soft_fail":false,"target_va":5380398224,"va":5376820800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3581962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e80aa83600","size":5,"soft_fail":false,"target_va":5380401776,"va":5376819809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3794227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e833e53900","size":5,"soft_fail":false,"target_va":5380621856,"va":5376827624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5113338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8fa054e00","size":5,"soft_fail":false,"target_va":5381934528,"va":5376821185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5109045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e835f54d00","size":5,"soft_fail":false,"target_va":5381934560,"va":5376825510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10727774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e85eb1a300","size":5,"soft_fail":false,"target_va":5387550144,"va":5376822365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11143472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e83009aa00","size":5,"soft_fail":false,"target_va":5387964304,"va":5376820827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t453187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e843ea0600","size":5,"soft_fail":false,"target_va":5377274800,"va":5376821608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34715061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8b5b51102","size":5,"soft_fail":false,"target_va":5411541584,"va":5376826518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34715630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8eeb71102","size":5,"soft_fail":false,"target_va":5411541808,"va":5376826173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34715627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8ebb71102","size":5,"soft_fail":false,"target_va":5411541776,"va":5376826144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34715337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8c9b61102","size":5,"soft_fail":false,"target_va":5411542064,"va":5376826722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34715613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8ddb71102","size":5,"soft_fail":false,"target_va":5411541904,"va":5376826286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8e4faffff","size":5,"soft_fail":false,"target_va":5376817760,"va":5376819063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34724215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e877d91102","size":5,"soft_fail":false,"target_va":5411543360,"va":5376819140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t486352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8d06b0700","size":5,"soft_fail":false,"target_va":5377307232,"va":5376820875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2485518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e80eed2500","size":5,"soft_fail":false,"target_va":5379305584,"va":5376820061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3581019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e85ba43600","size":5,"soft_fail":false,"target_va":5380401520,"va":5376820496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::gothic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11308135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8678cac00","size":5,"soft_fail":false,"target_va":5388134016,"va":5376825876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4565939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8b3ab4500","size":5,"soft_fail":false,"target_va":5381385536,"va":5376819592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4571128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8f8bf4500","size":5,"soft_fail":false,"target_va":5381390640,"va":5376819507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4566785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e801af4500","size":5,"soft_fail":false,"target_va":5381386480,"va":5376819690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4565662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e89eaa4500","size":5,"soft_fail":false,"target_va":5381384704,"va":5376819037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34818488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8b8491302","size":5,"soft_fail":false,"target_va":5411644800,"va":5376826307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34710593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e841a41102","size":5,"soft_fail":false,"target_va":5411537104,"va":5376826506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t426881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e881830600","size":5,"soft_fail":false,"target_va":5377249968,"va":5376823082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t430199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e877900600","size":5,"soft_fail":false,"target_va":5377251504,"va":5376821300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t191992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8f8ed0200","size":5,"soft_fail":false,"target_va":5377012992,"va":5376820995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5090821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e805ae4d00","size":5,"soft_fail":false,"target_va":5381916256,"va":5376825430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5065207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376818912,"raw_hex":"e8f7494d00","size":5,"soft_fail":false,"target_va":5381886416,"va":5376821204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":429,"record_offset":10364,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":429,"record_offset":10196,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":429,"record_offset":10252,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":429,"record_offset":5948,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":429,"record_offset":5920,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":4612,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":9728,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":10556,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":11940,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":5864,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":429,"record_offset":14148,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":429,"record_offset":14176,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":429,"record_offset":9980,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":429,"record_offset":10168,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":429,"record_offset":16164,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":429,"record_offset":15740,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":429,"record_offset":15980,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":429,"record_offset":14832,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":429,"record_offset":9892,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":3628,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":2192,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":2652,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":429,"record_offset":8652,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":429,"record_offset":14920,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":429,"record_offset":15404,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25101,"source_line":99,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":429,"record_offset":10336,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":429,"record_offset":15128,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25100,"source_line":170,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":429,"record_offset":10308,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":429,"record_offset":15712,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10295,"source_line":74,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":429,"record_offset":10280,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":429,"record_offset":10436,"parent_record_offset":10404,"ancestor_record_offset":1924,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":429,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":1924,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":429,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":1924,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":429,"record_offset":11760,"parent_record_offset":10596,"ancestor_record_offset":1924,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":429,"record_offset":17268,"parent_record_offset":17244,"ancestor_record_offset":1924,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":429,"record_offset":10404,"parent_record_offset":10364,"ancestor_record_offset":1924,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":429,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":1924,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":1924,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":1924,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":3992,"parent_record_offset":3964,"ancestor_record_offset":1924,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":1924,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":1924,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":4108,"parent_record_offset":3936,"ancestor_record_offset":1924,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":1924,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":429,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":1924,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":429,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":429,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":1924,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":429,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":1924,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":429,"record_offset":6452,"parent_record_offset":6416,"ancestor_record_offset":1924,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":429,"record_offset":14620,"parent_record_offset":14200,"ancestor_record_offset":1924,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":429,"record_offset":14528,"parent_record_offset":14472,"ancestor_record_offset":1924,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":429,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":1924,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":429,"record_offset":14328,"parent_record_offset":14244,"ancestor_record_offset":1924,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":429,"record_offset":14392,"parent_record_offset":14200,"ancestor_record_offset":1924,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":429,"record_offset":15012,"parent_record_offset":14920,"ancestor_record_offset":1924,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":429,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":1924,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":429,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":1924,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":429,"record_offset":8432,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":429,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":1924,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":429,"record_offset":2352,"parent_record_offset":2320,"ancestor_record_offset":1924,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":429,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":1924,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":429,"record_offset":6232,"parent_record_offset":6204,"ancestor_record_offset":1924,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":429,"record_offset":16012,"parent_record_offset":15980,"ancestor_record_offset":1924,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":429,"record_offset":10132,"parent_record_offset":10004,"ancestor_record_offset":1924,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":429,"record_offset":14860,"parent_record_offset":14832,"ancestor_record_offset":1924,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":429,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":1924,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":429,"record_offset":3780,"parent_record_offset":3736,"ancestor_record_offset":1924,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":429,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":1924,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":429,"record_offset":14952,"parent_record_offset":14920,"ancestor_record_offset":1924,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":429,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":1924,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":429,"record_offset":6644,"parent_record_offset":5948,"ancestor_record_offset":1924,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":429,"record_offset":8608,"parent_record_offset":7552,"ancestor_record_offset":1924,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":429,"record_offset":9792,"parent_record_offset":9764,"ancestor_record_offset":1924,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":429,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":1924,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":429,"record_offset":4568,"parent_record_offset":3736,"ancestor_record_offset":1924,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":4172,"parent_record_offset":3884,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":429,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":1924,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1924,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1924,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":1924,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":1924,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1924,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1924,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":1924,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":2968,"parent_record_offset":2852,"ancestor_record_offset":1924,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1924,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":1924,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":2820,"parent_record_offset":2788,"ancestor_record_offset":1924,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":2788,"parent_record_offset":2752,"ancestor_record_offset":1924,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1924,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1924,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1924,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":3292,"parent_record_offset":2752,"ancestor_record_offset":1924,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2292,"parent_record_offset":2256,"ancestor_record_offset":1924,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":1924,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":1924,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2548,"parent_record_offset":2452,"ancestor_record_offset":1924,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1924,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2228,"parent_record_offset":2192,"ancestor_record_offset":1924,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1924,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":2852,"parent_record_offset":2820,"ancestor_record_offset":1924,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":3136,"parent_record_offset":2852,"ancestor_record_offset":1924,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":3108,"parent_record_offset":2852,"ancestor_record_offset":1924,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":2912,"parent_record_offset":2852,"ancestor_record_offset":1924,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":429,"record_offset":11788,"parent_record_offset":11760,"ancestor_record_offset":1924,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":429,"record_offset":17304,"parent_record_offset":17268,"ancestor_record_offset":1924,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":429,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":1924,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":429,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":1924,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":429,"record_offset":17360,"parent_record_offset":17268,"ancestor_record_offset":1924,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":429,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":1924,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":429,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":1924,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":10596,"parent_record_offset":10556,"ancestor_record_offset":1924,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":9764,"parent_record_offset":9728,"ancestor_record_offset":1924,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":11980,"parent_record_offset":11940,"ancestor_record_offset":1924,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":11532,"parent_record_offset":11504,"ancestor_record_offset":1924,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":1924,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":9824,"parent_record_offset":9792,"ancestor_record_offset":1924,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":429,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":1924,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":429,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":1924,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":429,"record_offset":14200,"parent_record_offset":14176,"ancestor_record_offset":1924,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":429,"record_offset":14648,"parent_record_offset":14200,"ancestor_record_offset":1924,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":429,"record_offset":10064,"parent_record_offset":10032,"ancestor_record_offset":1924,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":429,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":1924,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":429,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":1924,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":429,"record_offset":14272,"parent_record_offset":14244,"ancestor_record_offset":1924,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":429,"record_offset":14244,"parent_record_offset":14200,"ancestor_record_offset":1924,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":429,"record_offset":6168,"parent_record_offset":6132,"ancestor_record_offset":1924,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":429,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":1924,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":429,"record_offset":6300,"parent_record_offset":6132,"ancestor_record_offset":1924,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":429,"record_offset":6132,"parent_record_offset":6080,"ancestor_record_offset":1924,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":7848,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":6332,"parent_record_offset":6080,"ancestor_record_offset":1924,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":8460,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":6416,"parent_record_offset":6388,"ancestor_record_offset":1924,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":6388,"parent_record_offset":6080,"ancestor_record_offset":1924,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":429,"record_offset":7932,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":429,"record_offset":7820,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":429,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":1924,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":1924,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":1924,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":8316,"parent_record_offset":8148,"ancestor_record_offset":1924,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":1924,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":1924,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":429,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":1924,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":8256,"parent_record_offset":8172,"ancestor_record_offset":1924,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":1924,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":6488,"parent_record_offset":6080,"ancestor_record_offset":1924,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":8552,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":6080,"parent_record_offset":6040,"ancestor_record_offset":1924,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":1924,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":1924,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":7688,"parent_record_offset":7628,"ancestor_record_offset":1924,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":1924,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":7792,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":6040,"parent_record_offset":6004,"ancestor_record_offset":1924,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":429,"record_offset":8148,"parent_record_offset":7736,"ancestor_record_offset":1924,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":429,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":1924,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":3664,"parent_record_offset":3628,"ancestor_record_offset":1924,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":1924,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":5192,"parent_record_offset":4932,"ancestor_record_offset":1924,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":429,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":1924,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":3884,"parent_record_offset":3736,"ancestor_record_offset":1924,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":4144,"parent_record_offset":3884,"ancestor_record_offset":1924,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":429,"record_offset":7580,"parent_record_offset":7552,"ancestor_record_offset":1924,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":429,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":1924,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":429,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":1924,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":429,"record_offset":10096,"parent_record_offset":10064,"ancestor_record_offset":1924,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":429,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":1924,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":3736,"parent_record_offset":3700,"ancestor_record_offset":1924,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":3700,"parent_record_offset":3664,"ancestor_record_offset":1924,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00429.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::arrow_slit::arrow_slit_profile_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6210
// 首条位置: {"module":435,"inlinee":25104}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":26960,"parent_record_offset":26932,"ancestor_record_offset":26932,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":27112,"parent_record_offset":26932,"ancestor_record_offset":26932,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":27016,"parent_record_offset":26992,"ancestor_record_offset":26932,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":27040,"parent_record_offset":27016,"ancestor_record_offset":26932,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":435,"record_offset":26992,"parent_record_offset":26960,"ancestor_record_offset":26932,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":435,"record_offset":27136,"parent_record_offset":27112,"ancestor_record_offset":26932,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":435,"record_offset":27160,"parent_record_offset":27136,"ancestor_record_offset":26932,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":435,"record_offset":27068,"parent_record_offset":27040,"ancestor_record_offset":26932,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00435.jsonl.gz"}
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
// 完整结构及原始类型记录: arrow_slit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: arrow_slit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: arrow_slit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::arrow_slit::arrow_slit_profile_curve
// 信息 ID: e42271fda876ac2abafd06e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=435, inline_record=26932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::arrow_slit::generate_arrow_slits
// 信息 ID: a5992790487a0fa06d1e1277
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 88d0867f7d3735af30de04ed: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x173f0a -> 0x7bbee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> > > @ 0x440f8e -> 0x7bbee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
