// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/wall_constructor.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d682d7f308006d7845fd19868057793a3845bb50766293606842cf28c4c8eff8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_constructor.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 187, 394, 401, 406, 408, 1415, 1417, 1431, 1444, 1452, 1462, 3024, 4061, 4067, 4068
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":401,"record_offset":2052,"rva":7772000,"va":5376481120}
// 编译实例 RVAs: 0x769760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1400729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e867a0eaff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376482212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1404716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8d490eaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376486215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-966723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8bd3ff1ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5376482750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1249669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e885111300","size":5,"soft_fail":false,"target_va":5377742176,"va":5376492502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5680
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1414025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e889931500","size":5,"soft_fail":false,"target_va":5377906304,"va":5376492274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5397987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8e35d5200","size":5,"soft_fail":false,"target_va":5381883040,"va":5376485048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5443291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8db0e5300","size":5,"soft_fail":false,"target_va":5381934528,"va":5376491232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5442621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e83d0c5300","size":5,"soft_fail":false,"target_va":5381934560,"va":5376491934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11062774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8f6cda800","size":5,"soft_fail":false,"target_va":5387553808,"va":5376491029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122cf10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11273911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8b706ac00","size":5,"soft_fail":false,"target_va":5387767568,"va":5376493652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123fc70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11361894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8665ead00","size":5,"soft_fail":false,"target_va":5387844720,"va":5376482821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35362933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e875981b02","size":5,"soft_fail":false,"target_va":5411847200,"va":5376484262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35364758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8969f1b02","size":5,"soft_fail":false,"target_va":5411847776,"va":5376483013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35361497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8d9921b02","size":5,"soft_fail":false,"target_va":5411848144,"va":5376486642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924060
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35361614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e84e931b02","size":5,"soft_fail":false,"target_va":5411848288,"va":5376486669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35047187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e813c71602","size":5,"soft_fail":false,"target_va":5411540992,"va":5376493800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11360081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e85157ad00","size":5,"soft_fail":false,"target_va":5387850016,"va":5376489930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2128584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e83885dfff","size":5,"soft_fail":false,"target_va":5374355568,"va":5376484147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34855140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8e4d81302","size":5,"soft_fail":false,"target_va":5411339744,"va":5376484599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35057793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e881f01602","size":5,"soft_fail":false,"target_va":5411540963,"va":5376483165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35048180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8f4ca1602","size":5,"soft_fail":false,"target_va":5411541584,"va":5376493399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35048924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8dccd1602","size":5,"soft_fail":false,"target_va":5411542422,"va":5376493493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35048677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8e5cc1602","size":5,"soft_fail":false,"target_va":5411542064,"va":5376493382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35048546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e862cc1602","size":5,"soft_fail":false,"target_va":5411542296,"va":5376493745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35049382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8a6cf1602","size":5,"soft_fail":false,"target_va":5411543088,"va":5376493701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11283175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8e72aac00","size":5,"soft_fail":false,"target_va":5387777040,"va":5376493860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e829fbffff","size":5,"soft_fail":false,"target_va":5376480048,"va":5376481282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35061996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8ec001702","size":5,"soft_fail":false,"target_va":5411543360,"va":5376481359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8507170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e822cf8100","size":5,"soft_fail":false,"target_va":5384989936,"va":5376482761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5525790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e81e515400","size":5,"soft_fail":false,"target_va":5382009728,"va":5376483933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4903749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e845d34a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376481782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4908938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e88ae74a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376481697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4904312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e878d54a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376482163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4903439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e80fd24a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376481260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35147481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8d94e1802","size":5,"soft_fail":false,"target_va":5411640768,"va":5376493282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11067448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e838e0a800","size":5,"soft_fail":false,"target_va":5387564432,"va":5376496979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35043741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e89db91602","size":5,"soft_fail":false,"target_va":5411537104,"va":5376493358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11359154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8b253ad00","size":5,"soft_fail":false,"target_va":5387849008,"va":5376489849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11359612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e87c55ad00","size":5,"soft_fail":false,"target_va":5387850176,"va":5376490559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11358425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8d950ad00","size":5,"soft_fail":false,"target_va":5387849408,"va":5376490978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11355261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e87d44ad00","size":5,"soft_fail":false,"target_va":5387837712,"va":5376482446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11358517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e83551ad00","size":5,"soft_fail":false,"target_va":5387841264,"va":5376482742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11354035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8b33fad00","size":5,"soft_fail":false,"target_va":5387845824,"va":5376491784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11362325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e81560ad00","size":5,"soft_fail":false,"target_va":5387846400,"va":5376484070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11354607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8ef41ad00","size":5,"soft_fail":false,"target_va":5387846384,"va":5376491772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11361890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8625ead00","size":5,"soft_fail":false,"target_va":5387846000,"va":5376484105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11354603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8eb41ad00","size":5,"soft_fail":false,"target_va":5387846368,"va":5376491760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5396166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376481120,"raw_hex":"e8c6565200","size":5,"soft_fail":false,"target_va":5381881936,"va":5376485765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":5780,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":20860,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":11293,"source_line":1554,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":20920,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":7364,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":13374,"source_line":576,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":23964,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":14426,"source_line":2962,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":401,"record_offset":24820,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":11311,"source_line":92,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":9448,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8168,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":7308,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8196,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8440,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":7640,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":20104,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":11648,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":10392,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":401,"record_offset":23852,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":25192,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":4888,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":9236,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":401,"record_offset":8060,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":401,"record_offset":9532,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":9076,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":401,"record_offset":20020,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":6184,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":401,"record_offset":10448,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":401,"record_offset":20200,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":20488,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":7020,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":20516,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":4920,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":13300,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":4820,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":13472,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":25220,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":24940,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23664,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23936,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23608,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":10924,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":401,"record_offset":13632,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":3836,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":2404,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":2860,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":7612,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":401,"record_offset":13796,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24627,"source_line":12,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":401,"record_offset":20720,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24641,"source_line":45,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":7048,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24620,"source_line":56,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14296,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24630,"source_line":376,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":21364,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24643,"source_line":764,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":23340,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24648,"source_line":803,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":2052,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":2052,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":5812,"parent_record_offset":5780,"ancestor_record_offset":2052,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":12992,"parent_record_offset":12900,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21676,"parent_record_offset":21648,"ancestor_record_offset":2052,"inlinee":14988,"source_line":1304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":24016,"parent_record_offset":23988,"ancestor_record_offset":2052,"inlinee":11308,"source_line":1270,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21896,"parent_record_offset":21676,"ancestor_record_offset":2052,"inlinee":11310,"source_line":196,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":20984,"parent_record_offset":20960,"ancestor_record_offset":2052,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":20960,"parent_record_offset":20920,"ancestor_record_offset":2052,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":2052,"inlinee":11307,"source_line":1214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21648,"parent_record_offset":21364,"ancestor_record_offset":2052,"inlinee":24644,"source_line":1237,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21228,"parent_record_offset":21072,"ancestor_record_offset":2052,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21704,"parent_record_offset":21676,"ancestor_record_offset":2052,"inlinee":11309,"source_line":1358,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21072,"parent_record_offset":20920,"ancestor_record_offset":2052,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":20888,"parent_record_offset":20860,"ancestor_record_offset":2052,"inlinee":11294,"source_line":1146,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":7392,"parent_record_offset":7364,"ancestor_record_offset":2052,"inlinee":13375,"source_line":632,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21104,"parent_record_offset":21072,"ancestor_record_offset":2052,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::iter_mut::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":401,"record_offset":22192,"parent_record_offset":21364,"ancestor_record_offset":2052,"inlinee":24646,"source_line":157,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":21008,"parent_record_offset":20984,"ancestor_record_offset":2052,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":2052,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":2052,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":2052,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":2052,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":12928,"parent_record_offset":12900,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":12900,"parent_record_offset":12876,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8648,"parent_record_offset":8616,"ancestor_record_offset":2052,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":2052,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":2052,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":2052,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":2052,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":2052,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":12876,"parent_record_offset":12852,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":7796,"parent_record_offset":7756,"ancestor_record_offset":2052,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":2052,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":7124,"parent_record_offset":7048,"ancestor_record_offset":2052,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":2052,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8236,"parent_record_offset":8196,"ancestor_record_offset":2052,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":7096,"parent_record_offset":7048,"ancestor_record_offset":2052,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":2052,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8500,"parent_record_offset":8440,"ancestor_record_offset":2052,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":2052,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8472,"parent_record_offset":8440,"ancestor_record_offset":2052,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":8560,"parent_record_offset":8500,"ancestor_record_offset":2052,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":6316,"parent_record_offset":6280,"ancestor_record_offset":2052,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":6344,"parent_record_offset":6316,"ancestor_record_offset":2052,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":2052,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":2052,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":401,"record_offset":6916,"parent_record_offset":6852,"ancestor_record_offset":2052,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":401,"record_offset":12788,"parent_record_offset":11704,"ancestor_record_offset":2052,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":401,"record_offset":12616,"parent_record_offset":12588,"ancestor_record_offset":2052,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":401,"record_offset":6564,"parent_record_offset":6280,"ancestor_record_offset":2052,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":401,"record_offset":6248,"parent_record_offset":6216,"ancestor_record_offset":2052,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":401,"record_offset":6280,"parent_record_offset":6248,"ancestor_record_offset":2052,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":401,"record_offset":7824,"parent_record_offset":7796,"ancestor_record_offset":2052,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":401,"record_offset":7852,"parent_record_offset":7824,"ancestor_record_offset":2052,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":401,"record_offset":9708,"parent_record_offset":9676,"ancestor_record_offset":2052,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":401,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":2052,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":2052,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":2052,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":2052,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":2052,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":2052,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4316,"parent_record_offset":4144,"ancestor_record_offset":2052,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":401,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":2052,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":2052,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":2052,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":10520,"parent_record_offset":10496,"ancestor_record_offset":2052,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":8944,"parent_record_offset":8912,"ancestor_record_offset":2052,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":2052,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":401,"record_offset":25272,"parent_record_offset":25220,"ancestor_record_offset":2052,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":401,"record_offset":25244,"parent_record_offset":25220,"ancestor_record_offset":2052,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":14880,"parent_record_offset":14848,"ancestor_record_offset":2052,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":7236,"parent_record_offset":7196,"ancestor_record_offset":2052,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":2052,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":11260,"parent_record_offset":11236,"ancestor_record_offset":2052,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":2052,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":401,"record_offset":10972,"parent_record_offset":10924,"ancestor_record_offset":2052,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":401,"record_offset":15744,"parent_record_offset":15704,"ancestor_record_offset":2052,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":401,"record_offset":9616,"parent_record_offset":9560,"ancestor_record_offset":2052,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":401,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":2052,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":401,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":2052,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":401,"record_offset":9844,"parent_record_offset":9560,"ancestor_record_offset":2052,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":2052,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":9928,"parent_record_offset":9872,"ancestor_record_offset":2052,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":6700,"parent_record_offset":6660,"ancestor_record_offset":2052,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":401,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":2052,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":401,"record_offset":15092,"parent_record_offset":15060,"ancestor_record_offset":2052,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":2052,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15028,"parent_record_offset":14996,"ancestor_record_offset":2052,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":2052,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":2052,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":2052,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":401,"record_offset":8912,"parent_record_offset":8880,"ancestor_record_offset":2052,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":401,"record_offset":8880,"parent_record_offset":8500,"ancestor_record_offset":2052,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":2052,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":6824,"parent_record_offset":6700,"ancestor_record_offset":2052,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":21452,"parent_record_offset":21364,"ancestor_record_offset":2052,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":2052,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":401,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":2052,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":401,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":2052,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":2052,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":7908,"parent_record_offset":7880,"ancestor_record_offset":2052,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":2052,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":12960,"parent_record_offset":12928,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":401,"record_offset":2560,"parent_record_offset":2528,"ancestor_record_offset":2052,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":2052,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":12220,"parent_record_offset":12144,"ancestor_record_offset":2052,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":401,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":2052,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":12180,"parent_record_offset":12144,"ancestor_record_offset":2052,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":3988,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":2052,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":401,"record_offset":9772,"parent_record_offset":9740,"ancestor_record_offset":2052,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":401,"record_offset":9740,"parent_record_offset":9708,"ancestor_record_offset":2052,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":401,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":2052,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":9648,"parent_record_offset":9616,"ancestor_record_offset":2052,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":11200,"parent_record_offset":10972,"ancestor_record_offset":2052,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":10092,"parent_record_offset":9928,"ancestor_record_offset":2052,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":11284,"parent_record_offset":11260,"ancestor_record_offset":2052,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":16280,"parent_record_offset":15704,"ancestor_record_offset":2052,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":2052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":4776,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":2052,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":4380,"parent_record_offset":4092,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":2052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":6956,"parent_record_offset":6660,"ancestor_record_offset":2052,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2052,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2052,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":2052,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":10328,"parent_record_offset":10304,"ancestor_record_offset":2052,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":2052,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":2052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":24392,"parent_record_offset":23988,"ancestor_record_offset":2052,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":22048,"parent_record_offset":21648,"ancestor_record_offset":2052,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":2052,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22548,"parent_record_offset":22524,"ancestor_record_offset":2052,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":20132,"parent_record_offset":20104,"ancestor_record_offset":2052,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":20160,"parent_record_offset":20132,"ancestor_record_offset":2052,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22524,"parent_record_offset":22500,"ancestor_record_offset":2052,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22500,"parent_record_offset":22464,"ancestor_record_offset":2052,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22464,"parent_record_offset":22432,"ancestor_record_offset":2052,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":401,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":2052,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2052,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":10020,"parent_record_offset":9988,"ancestor_record_offset":2052,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2052,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":401,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":2052,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":2052,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":3528,"parent_record_offset":3500,"ancestor_record_offset":2052,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":2052,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":8088,"parent_record_offset":8060,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":16756,"parent_record_offset":16696,"ancestor_record_offset":2052,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":3176,"parent_record_offset":3060,"ancestor_record_offset":2052,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":9988,"parent_record_offset":9956,"ancestor_record_offset":2052,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":22228,"parent_record_offset":22192,"ancestor_record_offset":2052,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$191::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":2052,"inlinee":24622,"source_line":392,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":13516,"parent_record_offset":13472,"ancestor_record_offset":2052,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":401,"record_offset":7196,"parent_record_offset":7160,"ancestor_record_offset":2052,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":401,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2052,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":401,"record_offset":2888,"parent_record_offset":2860,"ancestor_record_offset":2052,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":401,"record_offset":3028,"parent_record_offset":2996,"ancestor_record_offset":2052,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":401,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":2052,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2052,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2052,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":3500,"parent_record_offset":2960,"ancestor_record_offset":2052,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":2052,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":2052,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2660,"parent_record_offset":2628,"ancestor_record_offset":2052,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2756,"parent_record_offset":2660,"ancestor_record_offset":2052,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2052,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2440,"parent_record_offset":2404,"ancestor_record_offset":2052,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2052,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":3060,"parent_record_offset":3028,"ancestor_record_offset":2052,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":3344,"parent_record_offset":3060,"ancestor_record_offset":2052,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":3316,"parent_record_offset":3060,"ancestor_record_offset":2052,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":3120,"parent_record_offset":3060,"ancestor_record_offset":2052,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":23900,"parent_record_offset":23876,"ancestor_record_offset":2052,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":2052,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23748,"parent_record_offset":23716,"ancestor_record_offset":2052,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23716,"parent_record_offset":23688,"ancestor_record_offset":2052,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23688,"parent_record_offset":23664,"ancestor_record_offset":2052,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23152,"parent_record_offset":23124,"ancestor_record_offset":2052,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23180,"parent_record_offset":23152,"ancestor_record_offset":2052,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23264,"parent_record_offset":23152,"ancestor_record_offset":2052,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":11236,"parent_record_offset":10924,"ancestor_record_offset":2052,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14484,"parent_record_offset":14400,"ancestor_record_offset":2052,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":401,"record_offset":12144,"parent_record_offset":12108,"ancestor_record_offset":2052,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":3872,"parent_record_offset":3836,"ancestor_record_offset":2052,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":4984,"parent_record_offset":4952,"ancestor_record_offset":2052,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":5500,"parent_record_offset":5240,"ancestor_record_offset":2052,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":401,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":2052,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":4092,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":4352,"parent_record_offset":4092,"ancestor_record_offset":2052,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":13896,"parent_record_offset":13844,"ancestor_record_offset":2052,"inlinee":12960,"source_line":172,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":13868,"parent_record_offset":13844,"ancestor_record_offset":2052,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":23780,"parent_record_offset":23748,"ancestor_record_offset":2052,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::trunc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":2052,"inlinee":12961,"source_line":149,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":401,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":2052,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":401,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":2052,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":401,"record_offset":3908,"parent_record_offset":3872,"ancestor_record_offset":2052,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":401,"record_offset":13844,"parent_record_offset":13796,"ancestor_record_offset":2052,"inlinee":24629,"source_line":82,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":401,"record_offset":20572,"parent_record_offset":20544,"ancestor_record_offset":2052,"inlinee":24641,"source_line":45,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":2052,"inlinee":24618,"source_line":142,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":12108,"parent_record_offset":12072,"ancestor_record_offset":2052,"inlinee":24625,"source_line":335,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":2052,"inlinee":24634,"source_line":372,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":20544,"parent_record_offset":20516,"ancestor_record_offset":2052,"inlinee":24640,"source_line":525,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":2052,"inlinee":24647,"source_line":627,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":401,"record_offset":14996,"parent_record_offset":14964,"ancestor_record_offset":2052,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":23064,"parent_record_offset":23036,"ancestor_record_offset":2052,"inlinee":24648,"source_line":803,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16516,"parent_record_offset":16436,"ancestor_record_offset":2052,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":15388,"parent_record_offset":14296,"ancestor_record_offset":2052,"inlinee":24639,"source_line":1347,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14364,"parent_record_offset":14296,"ancestor_record_offset":2052,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14400,"parent_record_offset":14364,"ancestor_record_offset":2052,"inlinee":24631,"source_line":243,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":2052,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":2052,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":2052,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":23124,"parent_record_offset":23064,"ancestor_record_offset":2052,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":14848,"parent_record_offset":14400,"ancestor_record_offset":2052,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$5::default
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x37CA
// 首条位置: {"module":187,"inlinee":14282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x602C
// 首条位置: {"module":401,"inlinee":24620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":7124,"parent_record_offset":7048,"ancestor_record_offset":7048,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":7096,"parent_record_offset":7048,"ancestor_record_offset":7048,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":401,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":7048,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":7236,"parent_record_offset":7196,"ancestor_record_offset":7048,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":401,"record_offset":7196,"parent_record_offset":7160,"ancestor_record_offset":7048,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115DA
// 首条位置: {"module":1462,"inlinee":71130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1462,"record_offset":616,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1462,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":29552,"source_line":1182,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1462,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":532,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1462,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":532,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1462,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":532,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1462,"record_offset":656,"parent_record_offset":616,"ancestor_record_offset":532,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":532,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1462,"record_offset":776,"parent_record_offset":616,"ancestor_record_offset":532,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=45
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11547
// 首条位置: {"module":1444,"inlinee":70983}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1444,"record_offset":7400,"parent_record_offset":7380,"ancestor_record_offset":7380,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x602A
// 首条位置: {"module":401,"inlinee":24618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":401,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6728,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":6728,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6031
// 首条位置: {"module":401,"inlinee":24625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":401,"record_offset":12144,"parent_record_offset":12108,"ancestor_record_offset":12108,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":12264,"parent_record_offset":12144,"ancestor_record_offset":12108,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":12220,"parent_record_offset":12144,"ancestor_record_offset":12108,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":12180,"parent_record_offset":12144,"ancestor_record_offset":12108,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x603A
// 首条位置: {"module":401,"inlinee":24634}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14724,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6036
// 首条位置: {"module":401,"inlinee":24630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":401,"record_offset":15416,"parent_record_offset":14296,"ancestor_record_offset":14296,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":15388,"parent_record_offset":14296,"ancestor_record_offset":14296,"inlinee":24639,"source_line":1347,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14364,"parent_record_offset":14296,"ancestor_record_offset":14296,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":401,"record_offset":14880,"parent_record_offset":14848,"ancestor_record_offset":14296,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":16340,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14296,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":14296,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14296,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":401,"record_offset":15092,"parent_record_offset":15060,"ancestor_record_offset":14296,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15028,"parent_record_offset":14996,"ancestor_record_offset":14296,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14296,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14296,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":15500,"parent_record_offset":15416,"ancestor_record_offset":14296,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":15472,"parent_record_offset":15416,"ancestor_record_offset":14296,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":14820,"parent_record_offset":14400,"ancestor_record_offset":14296,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":14296,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14296,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":401,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14296,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":15164,"parent_record_offset":15124,"ancestor_record_offset":14296,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":15124,"parent_record_offset":15092,"ancestor_record_offset":14296,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":16756,"parent_record_offset":16696,"ancestor_record_offset":16340,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":14296,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":401,"record_offset":14484,"parent_record_offset":14400,"ancestor_record_offset":14296,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":401,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14296,"inlinee":24634,"source_line":372,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":401,"record_offset":14996,"parent_record_offset":14964,"ancestor_record_offset":14296,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16516,"parent_record_offset":16436,"ancestor_record_offset":16340,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14400,"parent_record_offset":14364,"ancestor_record_offset":14296,"inlinee":24631,"source_line":243,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":14296,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":401,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14296,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":14848,"parent_record_offset":14400,"ancestor_record_offset":14296,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 525
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6040
// 首条位置: {"module":401,"inlinee":24640}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":401,"record_offset":20572,"parent_record_offset":20544,"ancestor_record_offset":20544,"inlinee":24641,"source_line":45,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":20624,"parent_record_offset":20572,"ancestor_record_offset":20544,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6047
// 首条位置: {"module":401,"inlinee":24647}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":23064,"parent_record_offset":23036,"ancestor_record_offset":23036,"inlinee":24648,"source_line":803,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":23036,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":23036,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23152,"parent_record_offset":23124,"ancestor_record_offset":23036,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23180,"parent_record_offset":23152,"ancestor_record_offset":23036,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":23264,"parent_record_offset":23152,"ancestor_record_offset":23036,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":23096,"parent_record_offset":23064,"ancestor_record_offset":23036,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":23124,"parent_record_offset":23064,"ancestor_record_offset":23036,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::split_brick_by_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 764
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6043
// 首条位置: {"module":401,"inlinee":24643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21648,"parent_record_offset":21364,"ancestor_record_offset":21364,"inlinee":24644,"source_line":1237,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter_mut::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":401,"record_offset":22192,"parent_record_offset":21364,"ancestor_record_offset":21364,"inlinee":24646,"source_line":157,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21592,"parent_record_offset":21364,"ancestor_record_offset":21364,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":21452,"parent_record_offset":21364,"ancestor_record_offset":21364,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22432,"parent_record_offset":21364,"ancestor_record_offset":21364,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21676,"parent_record_offset":21648,"ancestor_record_offset":21364,"inlinee":14988,"source_line":1304,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21896,"parent_record_offset":21676,"ancestor_record_offset":21364,"inlinee":11310,"source_line":196,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21792,"parent_record_offset":21764,"ancestor_record_offset":21364,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21920,"parent_record_offset":21896,"ancestor_record_offset":21364,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21704,"parent_record_offset":21676,"ancestor_record_offset":21364,"inlinee":11309,"source_line":1358,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21740,"parent_record_offset":21704,"ancestor_record_offset":21364,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":401,"record_offset":21764,"parent_record_offset":21740,"ancestor_record_offset":21364,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":21824,"parent_record_offset":21792,"ancestor_record_offset":21364,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":21952,"parent_record_offset":21920,"ancestor_record_offset":21364,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":21364,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":22000,"parent_record_offset":21976,"ancestor_record_offset":21364,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":401,"record_offset":21976,"parent_record_offset":21952,"ancestor_record_offset":21364,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":21364,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":21364,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":21364,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":21364,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":21364,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":22096,"parent_record_offset":22072,"ancestor_record_offset":21364,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":22300,"parent_record_offset":22276,"ancestor_record_offset":21364,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":22368,"parent_record_offset":22340,"ancestor_record_offset":21364,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":22048,"parent_record_offset":21648,"ancestor_record_offset":21364,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":21364,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22548,"parent_record_offset":22524,"ancestor_record_offset":21364,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22524,"parent_record_offset":22500,"ancestor_record_offset":21364,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22500,"parent_record_offset":22464,"ancestor_record_offset":21364,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":22464,"parent_record_offset":22432,"ancestor_record_offset":21364,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":401,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":21364,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":21364,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":22228,"parent_record_offset":22192,"ancestor_record_offset":21364,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":22252,"parent_record_offset":22228,"ancestor_record_offset":21364,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00401.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_constructor.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::wall_constructor::WallConstructor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_constructor.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-integrate/B-core/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::wall_constructor::WallConstructor", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-production/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/wall_constructor.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_constructor.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve
// 信息 ID: 74b47e15e8435edd273626fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x4993df -> 0x769760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49f6eb -> 0x769760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x75925f -> 0x769760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x776dd0 -> 0x769760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x77f72a -> 0x769760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x12114e7 -> 0x769760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
// 信息 ID: 744971e5d00597a3291a4691
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=7048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
// 信息 ID: 38f1d003349de0a7989674d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=6728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$2
// 信息 ID: eab040fd7fe844fdf4acb6c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=12108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
// 信息 ID: dbd0cbc755c984aa0a1d42a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14724; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=15296; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17248; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17776; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=18788; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=19316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
// 信息 ID: 8dd3c68a6774f07326b83d37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=14296; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=16340; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=17900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$6
// 信息 ID: d9571a23eb209ad036b467a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=20544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$8
// 信息 ID: 3faa297a1cf125ff4d7583e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=23036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$0
// 信息 ID: 38e8be7e991c83c8ed7af6ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::normalize_holes", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0 <- country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1462, inline_record=532; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1462, inline_record=1048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1462, inline_record=1444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$1
// 信息 ID: 639427fb5a0d03a35d7a661a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::normalize_holes", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::driftsort_main<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::drift::sort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=7380; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=7480; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=7644; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=7764; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=7952; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=8288; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=12808; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1444, inline_record=13224; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$5::default
// 信息 ID: b48061a489e7363375eada1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true>; module=187, inline_record=91832; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=187, inline_record=149484; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=9920; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=406, inline_record=5312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals; module=408, inline_record=19184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=7548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::split_brick_by_holes
// 信息 ID: acd20c618c6f2a72530c826a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=401, inline_record=21364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
