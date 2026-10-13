// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/derive_decorator_subtype.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 71673c5459962b21d53fa1feafd39fff8897089f62bdfe18b93bb0cf8ce6ea79
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: derive_decorator_subtype.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 456, 1546, 1571
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
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
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtypes
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":456,"record_offset":2900,"rva":8465184,"va":5377174304}
// 编译实例 RVAs: 0x812b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8222949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e81b8782ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5377176672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3548616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e838dac9ff","size":5,"soft_fail":false,"target_va":5373629488,"va":5377178099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1662019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8bda3e6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377178046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t132503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e897050200","size":5,"soft_fail":false,"target_va":5377309600,"va":5377177092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408392d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t155369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8e95e0200","size":5,"soft_fail":false,"target_va":5377331920,"va":5377176546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2129734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8467f2000","size":5,"soft_fail":false,"target_va":5379305728,"va":5377175989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2134790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e806932000","size":5,"soft_fail":false,"target_va":5379310240,"va":5377175445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3221118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e87e263100","size":5,"soft_fail":false,"target_va":5380398224,"va":5377177101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4712291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e863e74700","size":5,"soft_fail":false,"target_va":5381889520,"va":5377177224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4713183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8dfea4700","size":5,"soft_fail":false,"target_va":5381889824,"va":5377176636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10787170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e86299a400","size":5,"soft_fail":false,"target_va":5387964304,"va":5377177129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141266c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10828510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8de3aa500","size":5,"soft_fail":false,"target_va":5388004432,"va":5377175917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b79e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11157935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8af41aa00","size":5,"soft_fail":false,"target_va":5388335584,"va":5377177644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34672143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e80f0e1102","size":5,"soft_fail":false,"target_va":5411847200,"va":5377175052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34363788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e88c590c02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377177791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34364101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8c55a0c02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377177670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34364477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e83d5c0c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377177582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34364125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8dd5a0c02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377177774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e840faffff","size":5,"soft_fail":false,"target_va":5377172992,"va":5377174459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34368819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8336d0c02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377174536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2126969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e879742000","size":5,"soft_fail":false,"target_va":5379303920,"va":5377176946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2144092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e85cb72000","size":5,"soft_fail":false,"target_va":5379319216,"va":5377175119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3240529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e851723100","size":5,"soft_fail":false,"target_va":5380417056,"va":5377176522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3240781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e84d733100","size":5,"soft_fail":false,"target_va":5380417344,"va":5377176558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2616117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e835eb2700","size":5,"soft_fail":false,"target_va":5379792624,"va":5377176502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4210565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8853f4000","size":5,"soft_fail":false,"target_va":5381385536,"va":5377174966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4215754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8ca534000","size":5,"soft_fail":false,"target_va":5381390640,"va":5377174881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4209087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8bf394000","size":5,"soft_fail":false,"target_va":5381386480,"va":5377177388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4210266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e85a3e4000","size":5,"soft_fail":false,"target_va":5381384704,"va":5377174433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34467287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8d7ed0d02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377177508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10764270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8ee3fa400","size":5,"soft_fail":false,"target_va":5387942848,"va":5377178573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34359541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8f5480c02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377177558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::derive_decorator_subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e8b20b0000","size":5,"soft_fail":false,"target_va":5377178880,"va":5377175881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4713480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377174304,"raw_hex":"e808ec4700","size":5,"soft_fail":false,"target_va":5381890192,"va":5377176707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":456,"record_offset":5556,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":13764,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":456,"record_offset":10480,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":13796,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":456,"record_offset":6084,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":456,"record_offset":8628,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":456,"record_offset":6056,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":13584,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":10444,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":456,"record_offset":12544,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":456,"record_offset":10508,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":456,"record_offset":10564,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":456,"record_offset":13680,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":4572,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":3140,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":3596,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":5616,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":8572,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":7500,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":7832,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::derive_decorator_subtype::can_become_full_chimney
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":10648,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":25665,"source_line":301,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":456,"record_offset":10620,"parent_record_offset":2900,"ancestor_record_offset":2900,"inlinee":25129,"source_line":247,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":456,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":2900,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":4908,"parent_record_offset":4880,"ancestor_record_offset":2900,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":2900,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":2900,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":2900,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":2900,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":5052,"parent_record_offset":4880,"ancestor_record_offset":2900,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":456,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":2900,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":456,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":2900,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":2900,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":6800,"parent_record_offset":6764,"ancestor_record_offset":2900,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":456,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":2900,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":2900,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":456,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":2900,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":456,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":2900,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":456,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":2900,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":456,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":2900,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":456,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":2900,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11024,"parent_record_offset":10648,"ancestor_record_offset":2900,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11104,"parent_record_offset":11060,"ancestor_record_offset":2900,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11060,"parent_record_offset":11024,"ancestor_record_offset":2900,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":456,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":2900,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":456,"record_offset":3296,"parent_record_offset":3264,"ancestor_record_offset":2900,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":6600,"parent_record_offset":6568,"ancestor_record_offset":2900,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":2900,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":456,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":2900,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":456,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":2900,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":456,"record_offset":11292,"parent_record_offset":11244,"ancestor_record_offset":2900,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":456,"record_offset":13612,"parent_record_offset":13584,"ancestor_record_offset":2900,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":4724,"parent_record_offset":4680,"ancestor_record_offset":2900,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":2900,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":2900,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":7196,"parent_record_offset":6208,"ancestor_record_offset":2900,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":2900,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":7076,"parent_record_offset":6288,"ancestor_record_offset":2900,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":7256,"parent_record_offset":6184,"ancestor_record_offset":2900,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":11524,"parent_record_offset":11496,"ancestor_record_offset":2900,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":456,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":2900,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":456,"record_offset":5512,"parent_record_offset":4680,"ancestor_record_offset":2900,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":456,"record_offset":5116,"parent_record_offset":4828,"ancestor_record_offset":2900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":2900,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":2900,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":456,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":2900,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":456,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":2900,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":456,"record_offset":4192,"parent_record_offset":4164,"ancestor_record_offset":2900,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":456,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":2900,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":2900,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":2900,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":4264,"parent_record_offset":4236,"ancestor_record_offset":2900,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":3912,"parent_record_offset":3796,"ancestor_record_offset":2900,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":456,"record_offset":3264,"parent_record_offset":3236,"ancestor_record_offset":2900,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":456,"record_offset":3624,"parent_record_offset":3596,"ancestor_record_offset":2900,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":456,"record_offset":3764,"parent_record_offset":3732,"ancestor_record_offset":2900,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":456,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":2900,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":456,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":2900,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":456,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":2900,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":456,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":2900,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":456,"record_offset":4236,"parent_record_offset":3696,"ancestor_record_offset":2900,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3236,"parent_record_offset":3204,"ancestor_record_offset":2900,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3204,"parent_record_offset":3176,"ancestor_record_offset":2900,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3396,"parent_record_offset":3364,"ancestor_record_offset":2900,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3492,"parent_record_offset":3396,"ancestor_record_offset":2900,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":2900,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3176,"parent_record_offset":3140,"ancestor_record_offset":2900,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":456,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2900,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":456,"record_offset":3796,"parent_record_offset":3764,"ancestor_record_offset":2900,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":456,"record_offset":4080,"parent_record_offset":3796,"ancestor_record_offset":2900,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":456,"record_offset":4052,"parent_record_offset":3796,"ancestor_record_offset":2900,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":456,"record_offset":3856,"parent_record_offset":3796,"ancestor_record_offset":2900,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":456,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":2900,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":456,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":2900,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":456,"record_offset":11428,"parent_record_offset":11144,"ancestor_record_offset":2900,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":456,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":2900,"inlinee":24786,"source_line":746,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11616,"parent_record_offset":11496,"ancestor_record_offset":2900,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11496,"parent_record_offset":11460,"ancestor_record_offset":2900,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11460,"parent_record_offset":11144,"ancestor_record_offset":2900,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11556,"parent_record_offset":11524,"ancestor_record_offset":2900,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11216,"parent_record_offset":11144,"ancestor_record_offset":2900,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11352,"parent_record_offset":11244,"ancestor_record_offset":2900,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":2900,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":6496,"parent_record_offset":6456,"ancestor_record_offset":2900,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":6536,"parent_record_offset":6496,"ancestor_record_offset":2900,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":6644,"parent_record_offset":6456,"ancestor_record_offset":2900,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":6456,"parent_record_offset":6408,"ancestor_record_offset":2900,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":6680,"parent_record_offset":6408,"ancestor_record_offset":2900,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":6764,"parent_record_offset":6736,"ancestor_record_offset":2900,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":6736,"parent_record_offset":6408,"ancestor_record_offset":2900,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":456,"record_offset":5908,"parent_record_offset":5848,"ancestor_record_offset":2900,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":5640,"parent_record_offset":5616,"ancestor_record_offset":2900,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":2900,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":2900,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":2900,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":6984,"parent_record_offset":6340,"ancestor_record_offset":2900,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":6836,"parent_record_offset":6408,"ancestor_record_offset":2900,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":7632,"parent_record_offset":7572,"ancestor_record_offset":2900,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":6408,"parent_record_offset":6372,"ancestor_record_offset":2900,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":2900,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":2900,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":2900,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":2900,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5848,"parent_record_offset":5788,"ancestor_record_offset":2900,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":7604,"parent_record_offset":7572,"ancestor_record_offset":2900,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5820,"parent_record_offset":5788,"ancestor_record_offset":2900,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":2900,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5880,"parent_record_offset":5848,"ancestor_record_offset":2900,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5996,"parent_record_offset":5788,"ancestor_record_offset":2900,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":6372,"parent_record_offset":6340,"ancestor_record_offset":2900,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":2900,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":456,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":2900,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":4608,"parent_record_offset":4572,"ancestor_record_offset":2900,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":12608,"parent_record_offset":12576,"ancestor_record_offset":2900,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":13108,"parent_record_offset":12848,"ancestor_record_offset":2900,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":456,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":2900,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":4828,"parent_record_offset":4680,"ancestor_record_offset":2900,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":456,"record_offset":5088,"parent_record_offset":4828,"ancestor_record_offset":2900,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":10692,"parent_record_offset":10648,"ancestor_record_offset":2900,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":2900,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":456,"record_offset":11388,"parent_record_offset":11352,"ancestor_record_offset":2900,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":456,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":2900,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":456,"record_offset":4680,"parent_record_offset":4644,"ancestor_record_offset":2900,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":456,"record_offset":4644,"parent_record_offset":4608,"ancestor_record_offset":2900,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":11144,"parent_record_offset":11104,"ancestor_record_offset":2900,"inlinee":25667,"source_line":321,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11744,"parent_record_offset":11708,"ancestor_record_offset":2900,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":2900,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11668,"parent_record_offset":11616,"ancestor_record_offset":2900,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":2900,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":456,"record_offset":11708,"parent_record_offset":11144,"ancestor_record_offset":2900,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11888
// 首条位置: {"module":1546,"inlinee":71816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1546,"record_offset":33364,"parent_record_offset":33328,"ancestor_record_offset":33328,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1546,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":33328,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1546,"record_offset":33612,"parent_record_offset":33588,"ancestor_record_offset":33328,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1546,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":33328,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1546,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":33328,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1546,"record_offset":34168,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1546,"record_offset":33844,"parent_record_offset":33816,"ancestor_record_offset":33328,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1546,"record_offset":33816,"parent_record_offset":33788,"ancestor_record_offset":33328,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1546,"record_offset":33960,"parent_record_offset":33936,"ancestor_record_offset":33328,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1546,"record_offset":34124,"parent_record_offset":34100,"ancestor_record_offset":33328,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1546,"record_offset":33756,"parent_record_offset":33728,"ancestor_record_offset":33328,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1546,"record_offset":33788,"parent_record_offset":33756,"ancestor_record_offset":33328,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1546,"record_offset":33728,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1546,"record_offset":33588,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1546,"record_offset":34196,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1546,"record_offset":33672,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1546,"record_offset":34348,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1546,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":33328,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1546,"record_offset":33400,"parent_record_offset":33364,"ancestor_record_offset":33328,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1546,"record_offset":33436,"parent_record_offset":33400,"ancestor_record_offset":33328,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1546,"record_offset":34052,"parent_record_offset":33888,"ancestor_record_offset":33328,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1546,"record_offset":34388,"parent_record_offset":33436,"ancestor_record_offset":33328,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33992,"parent_record_offset":33908,"ancestor_record_offset":33328,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33936,"parent_record_offset":33908,"ancestor_record_offset":33328,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":34288,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33908,"parent_record_offset":33888,"ancestor_record_offset":33328,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33504,"parent_record_offset":33476,"ancestor_record_offset":33328,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33476,"parent_record_offset":33436,"ancestor_record_offset":33328,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33532,"parent_record_offset":33504,"ancestor_record_offset":33328,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":34320,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1546,"record_offset":33888,"parent_record_offset":33532,"ancestor_record_offset":33328,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":456,"record_offset":13968,"rva":8469760,"va":5377178880}
// 编译实例 RVAs: 0x813d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34361152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e8404f0c02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377180619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34361452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e86c500c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377180607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34361428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e854500c02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377180471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t848525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e88df20c00","size":5,"soft_fail":false,"target_va":5378028416,"va":5377179886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::WallId::is_valid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3280031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e89f0c3200","size":5,"soft_fail":false,"target_va":5380460320,"va":5377180284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2130420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e8f4812000","size":5,"soft_fail":false,"target_va":5379310144,"va":5377179719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2613548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e82ce12700","size":5,"soft_fail":false,"target_va":5379792624,"va":5377179071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2614485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e8d5e42700","size":5,"soft_fail":false,"target_va":5379793584,"va":5377179094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3217023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e87f163100","size":5,"soft_fail":false,"target_va":5380396720,"va":5377179692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::is_half_dormer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377178880,"raw_hex":"e801030000","size":5,"soft_fail":false,"target_va":5377180784,"va":5377180010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":20620,"parent_record_offset":13968,"ancestor_record_offset":13968,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":14168,"parent_record_offset":13968,"ancestor_record_offset":13968,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":18960,"parent_record_offset":13968,"ancestor_record_offset":13968,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":14132,"parent_record_offset":13968,"ancestor_record_offset":13968,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":13968,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13968,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":13968,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13968,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14448,"parent_record_offset":14348,"ancestor_record_offset":13968,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":13968,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":456,"record_offset":14648,"parent_record_offset":14316,"ancestor_record_offset":13968,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":456,"record_offset":14316,"parent_record_offset":14268,"ancestor_record_offset":13968,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":13968,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15120,"parent_record_offset":14268,"ancestor_record_offset":13968,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":13968,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15616,"parent_record_offset":14268,"ancestor_record_offset":13968,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":13968,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":456,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":13968,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15244,"parent_record_offset":15220,"ancestor_record_offset":13968,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":13968,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":13968,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":13968,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":13968,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":14860,"parent_record_offset":14268,"ancestor_record_offset":13968,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":456,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":13968,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":13968,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":14236,"parent_record_offset":14204,"ancestor_record_offset":13968,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":14268,"parent_record_offset":14236,"ancestor_record_offset":13968,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":13968,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":456,"record_offset":14708,"parent_record_offset":14680,"ancestor_record_offset":13968,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":13968,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":14592,"parent_record_offset":14316,"ancestor_record_offset":13968,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":15336,"parent_record_offset":15272,"ancestor_record_offset":13968,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":18500,"parent_record_offset":18476,"ancestor_record_offset":13968,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":13968,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":15400,"parent_record_offset":15220,"ancestor_record_offset":13968,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":14812,"parent_record_offset":14780,"ancestor_record_offset":13968,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":14348,"parent_record_offset":14316,"ancestor_record_offset":13968,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":13968,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":14388,"parent_record_offset":14348,"ancestor_record_offset":13968,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14680,"parent_record_offset":14268,"ancestor_record_offset":13968,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":13968,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":13968,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":13968,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":456,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":13968,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":19048,"parent_record_offset":19016,"ancestor_record_offset":13968,"inlinee":25673,"source_line":132,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":20588,"parent_record_offset":20564,"ancestor_record_offset":13968,"inlinee":25674,"source_line":133,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":15844,"parent_record_offset":15808,"ancestor_record_offset":13968,"inlinee":25670,"source_line":155,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":17444,"parent_record_offset":17412,"ancestor_record_offset":13968,"inlinee":25671,"source_line":183,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":13968,"inlinee":25672,"source_line":184,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":14204,"parent_record_offset":14168,"ancestor_record_offset":13968,"inlinee":25669,"source_line":211,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6449
// 首条位置: {"module":456,"inlinee":25673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":19076,"parent_record_offset":19048,"ancestor_record_offset":19048,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":19332,"parent_record_offset":19308,"ancestor_record_offset":19048,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":19380,"parent_record_offset":19356,"ancestor_record_offset":19048,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":19356,"parent_record_offset":19332,"ancestor_record_offset":19048,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":19048,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":19284,"parent_record_offset":19184,"ancestor_record_offset":19048,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":19720,"parent_record_offset":19696,"ancestor_record_offset":19048,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":456,"record_offset":19484,"parent_record_offset":19152,"ancestor_record_offset":19048,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":456,"record_offset":19152,"parent_record_offset":19104,"ancestor_record_offset":19048,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":19048,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":19956,"parent_record_offset":19104,"ancestor_record_offset":19048,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":19980,"parent_record_offset":19956,"ancestor_record_offset":19048,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":20408,"parent_record_offset":19104,"ancestor_record_offset":19048,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":19048,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":456,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":19048,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":19048,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":20056,"parent_record_offset":20032,"ancestor_record_offset":19048,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":19048,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":20032,"parent_record_offset":20004,"ancestor_record_offset":19048,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":19048,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":19696,"parent_record_offset":19104,"ancestor_record_offset":19048,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":456,"record_offset":20332,"parent_record_offset":20308,"ancestor_record_offset":19048,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":20308,"parent_record_offset":20284,"ancestor_record_offset":19048,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":19104,"parent_record_offset":19076,"ancestor_record_offset":19048,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":20480,"parent_record_offset":20456,"ancestor_record_offset":19048,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":456,"record_offset":19544,"parent_record_offset":19516,"ancestor_record_offset":19048,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":19452,"parent_record_offset":19428,"ancestor_record_offset":19048,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":19428,"parent_record_offset":19152,"ancestor_record_offset":19048,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19048,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19048,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":20152,"parent_record_offset":20128,"ancestor_record_offset":19048,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":20192,"parent_record_offset":20056,"ancestor_record_offset":19048,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":19648,"parent_record_offset":19616,"ancestor_record_offset":19048,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":19184,"parent_record_offset":19152,"ancestor_record_offset":19048,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":19048,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":19224,"parent_record_offset":19184,"ancestor_record_offset":19048,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":19516,"parent_record_offset":19104,"ancestor_record_offset":19048,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":19592,"parent_record_offset":19568,"ancestor_record_offset":19048,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":19616,"parent_record_offset":19592,"ancestor_record_offset":19048,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19048,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":456,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":19048,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x644A
// 首条位置: {"module":456,"inlinee":25674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6446
// 首条位置: {"module":456,"inlinee":25670}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":15876,"parent_record_offset":15844,"ancestor_record_offset":15844,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":15844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":16184,"parent_record_offset":16160,"ancestor_record_offset":15844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":16160,"parent_record_offset":16136,"ancestor_record_offset":15844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":15844,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":16088,"parent_record_offset":15988,"ancestor_record_offset":15844,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":16524,"parent_record_offset":16500,"ancestor_record_offset":15844,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":456,"record_offset":16288,"parent_record_offset":15956,"ancestor_record_offset":15844,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":456,"record_offset":15956,"parent_record_offset":15908,"ancestor_record_offset":15844,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":15844,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":16760,"parent_record_offset":15908,"ancestor_record_offset":15844,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":15844,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":17256,"parent_record_offset":15908,"ancestor_record_offset":15844,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":15844,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":456,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":15844,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":15844,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":16860,"parent_record_offset":16836,"ancestor_record_offset":15844,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":15844,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":16836,"parent_record_offset":16808,"ancestor_record_offset":15844,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":15844,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":16500,"parent_record_offset":15908,"ancestor_record_offset":15844,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":456,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":15844,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":15844,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":15908,"parent_record_offset":15876,"ancestor_record_offset":15844,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":15844,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":456,"record_offset":16348,"parent_record_offset":16320,"ancestor_record_offset":15844,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15844,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":16232,"parent_record_offset":15956,"ancestor_record_offset":15844,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":16976,"parent_record_offset":16912,"ancestor_record_offset":15844,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":17000,"parent_record_offset":16976,"ancestor_record_offset":15844,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":17040,"parent_record_offset":16860,"ancestor_record_offset":15844,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":16452,"parent_record_offset":16420,"ancestor_record_offset":15844,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":15988,"parent_record_offset":15956,"ancestor_record_offset":15844,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":15844,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":16028,"parent_record_offset":15988,"ancestor_record_offset":15844,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":16320,"parent_record_offset":15908,"ancestor_record_offset":15844,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":15844,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":15844,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":15844,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":456,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":15844,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6447
// 首条位置: {"module":456,"inlinee":25671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":17472,"parent_record_offset":17444,"ancestor_record_offset":17444,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":17444,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":17444,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":17444,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":17444,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":17680,"parent_record_offset":17580,"ancestor_record_offset":17444,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17444,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":456,"record_offset":17880,"parent_record_offset":17548,"ancestor_record_offset":17444,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":456,"record_offset":17548,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":17444,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":18352,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":18376,"parent_record_offset":18352,"ancestor_record_offset":17444,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":18804,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":18852,"parent_record_offset":18828,"ancestor_record_offset":17444,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":456,"record_offset":18752,"parent_record_offset":18728,"ancestor_record_offset":17444,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":17444,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":17444,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":17444,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":18428,"parent_record_offset":18400,"ancestor_record_offset":17444,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":17444,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":18092,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":456,"record_offset":18728,"parent_record_offset":18704,"ancestor_record_offset":17444,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":17444,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":17444,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":17444,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":456,"record_offset":17940,"parent_record_offset":17912,"ancestor_record_offset":17444,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":17848,"parent_record_offset":17824,"ancestor_record_offset":17444,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":17824,"parent_record_offset":17548,"ancestor_record_offset":17444,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":18524,"parent_record_offset":18500,"ancestor_record_offset":17444,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":18500,"parent_record_offset":18476,"ancestor_record_offset":17444,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":18548,"parent_record_offset":18524,"ancestor_record_offset":17444,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":18588,"parent_record_offset":18452,"ancestor_record_offset":17444,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":18044,"parent_record_offset":18012,"ancestor_record_offset":17444,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":17580,"parent_record_offset":17548,"ancestor_record_offset":17444,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":17444,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":17620,"parent_record_offset":17580,"ancestor_record_offset":17444,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":17912,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":17444,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":17444,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":17964,"parent_record_offset":17940,"ancestor_record_offset":17444,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":456,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":17444,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6448
// 首条位置: {"module":456,"inlinee":25672}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6445
// 首条位置: {"module":456,"inlinee":25669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":14236,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":14204,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":14204,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":456,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":14204,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":14204,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14448,"parent_record_offset":14348,"ancestor_record_offset":14204,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":456,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14204,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":456,"record_offset":14648,"parent_record_offset":14316,"ancestor_record_offset":14204,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":456,"record_offset":14316,"parent_record_offset":14268,"ancestor_record_offset":14204,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":14204,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15120,"parent_record_offset":14268,"ancestor_record_offset":14204,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":14204,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15616,"parent_record_offset":14268,"ancestor_record_offset":14204,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":456,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":14204,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":456,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":14204,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15244,"parent_record_offset":15220,"ancestor_record_offset":14204,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":14204,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":456,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":14204,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":14204,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":14204,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":14860,"parent_record_offset":14268,"ancestor_record_offset":14204,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":456,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":14204,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":456,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":14204,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":456,"record_offset":14268,"parent_record_offset":14236,"ancestor_record_offset":14204,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":456,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":14204,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":456,"record_offset":14708,"parent_record_offset":14680,"ancestor_record_offset":14204,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":14204,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":14592,"parent_record_offset":14316,"ancestor_record_offset":14204,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":456,"record_offset":15336,"parent_record_offset":15272,"ancestor_record_offset":14204,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14204,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":15400,"parent_record_offset":15220,"ancestor_record_offset":14204,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":14812,"parent_record_offset":14780,"ancestor_record_offset":14204,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":14348,"parent_record_offset":14316,"ancestor_record_offset":14204,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":14204,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":14388,"parent_record_offset":14348,"ancestor_record_offset":14204,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14680,"parent_record_offset":14268,"ancestor_record_offset":14204,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":14204,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":14204,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":456,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":14204,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":456,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":14204,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::is_half_dormer
// 记录形式: compiled_procedure; instances=1
// 源码行: 251
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":456,"record_offset":20876,"rva":8471664,"va":5377180784}
// 编译实例 RVAs: 0x814470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3217079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8b7163100","size":5,"soft_fail":false,"target_va":5380398224,"va":5377181140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4753015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e877864800","size":5,"soft_fail":false,"target_va":5381934560,"va":5377181540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10368966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8c6379e00","size":5,"soft_fail":false,"target_va":5387550144,"va":5377181173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14126a450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10837718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8d65ea500","size":5,"soft_fail":false,"target_va":5388018768,"va":5377181045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34359995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8bb4a0c02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377181776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34360295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8e74b0c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377181764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34360566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8f64c0c02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377181725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34360186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e87a4b0c02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377181713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t847582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8deee0c00","size":5,"soft_fail":false,"target_va":5378028512,"va":5377180925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t847534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8aeee0c00","size":5,"soft_fail":false,"target_va":5378028416,"va":5377180877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t847563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e8cbee0c00","size":5,"soft_fail":false,"target_va":5378028464,"va":5377180896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10400334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e84eb29e00","size":5,"soft_fail":false,"target_va":5387581248,"va":5377180909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4745077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e875674800","size":5,"soft_fail":false,"target_va":5381926064,"va":5377180982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4709225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377180784,"raw_hex":"e869db4700","size":5,"soft_fail":false,"target_va":5381890192,"va":5377180962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":21072,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":456,"record_offset":21108,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":21444,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":456,"record_offset":21044,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":456,"record_offset":21220,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":22776,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":10295,"source_line":74,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":22804,"parent_record_offset":20876,"ancestor_record_offset":20876,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":456,"record_offset":22392,"parent_record_offset":22364,"ancestor_record_offset":20876,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":21804,"parent_record_offset":21776,"ancestor_record_offset":20876,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":20876,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":20876,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":456,"record_offset":21832,"parent_record_offset":21804,"ancestor_record_offset":20876,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":456,"record_offset":22632,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":22088,"parent_record_offset":22052,"ancestor_record_offset":20876,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":22052,"parent_record_offset":22016,"ancestor_record_offset":20876,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":22688,"parent_record_offset":21444,"ancestor_record_offset":20876,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":22232,"parent_record_offset":22204,"ancestor_record_offset":20876,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":456,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":20876,"inlinee":25630,"source_line":266,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":456,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":20876,"inlinee":25632,"source_line":278,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":21284,"parent_record_offset":21220,"ancestor_record_offset":20876,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":456,"record_offset":21312,"parent_record_offset":21284,"ancestor_record_offset":20876,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":456,"record_offset":21156,"parent_record_offset":21132,"ancestor_record_offset":20876,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":22420,"parent_record_offset":22392,"ancestor_record_offset":20876,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":20876,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":21972,"parent_record_offset":21936,"ancestor_record_offset":20876,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":22016,"parent_record_offset":21972,"ancestor_record_offset":20876,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":21936,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":21776,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":22468,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":21872,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":456,"record_offset":21748,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":456,"record_offset":22364,"parent_record_offset":22336,"ancestor_record_offset":20876,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":20876,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":20876,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":22336,"parent_record_offset":22136,"ancestor_record_offset":20876,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":21556,"parent_record_offset":21524,"ancestor_record_offset":20876,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":22268,"parent_record_offset":22164,"ancestor_record_offset":20876,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":22204,"parent_record_offset":22164,"ancestor_record_offset":20876,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":22588,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":22164,"parent_record_offset":22136,"ancestor_record_offset":20876,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":20876,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":21584,"parent_record_offset":21524,"ancestor_record_offset":20876,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":21636,"parent_record_offset":21608,"ancestor_record_offset":20876,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":21720,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":22136,"parent_record_offset":21636,"ancestor_record_offset":20876,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":21476,"parent_record_offset":21444,"ancestor_record_offset":20876,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":456,"record_offset":21256,"parent_record_offset":21220,"ancestor_record_offset":20876,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":456,"record_offset":21348,"parent_record_offset":21220,"ancestor_record_offset":20876,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::can_become_full_chimney
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6441
// 首条位置: {"module":456,"inlinee":25665}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11024,"parent_record_offset":10648,"ancestor_record_offset":10648,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":10692,"parent_record_offset":10648,"ancestor_record_offset":10648,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":10648,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":456,"record_offset":10940,"parent_record_offset":10908,"ancestor_record_offset":10648,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":456,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":10648,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":10648,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11104,"parent_record_offset":11060,"ancestor_record_offset":10648,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":456,"record_offset":11060,"parent_record_offset":11024,"ancestor_record_offset":10648,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":10648,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":456,"record_offset":12108,"parent_record_offset":12076,"ancestor_record_offset":10648,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":456,"record_offset":11292,"parent_record_offset":11244,"ancestor_record_offset":10648,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":11524,"parent_record_offset":11496,"ancestor_record_offset":10648,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":456,"record_offset":10976,"parent_record_offset":10824,"ancestor_record_offset":10648,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":456,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":10648,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":456,"record_offset":11428,"parent_record_offset":11144,"ancestor_record_offset":10648,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11616,"parent_record_offset":11496,"ancestor_record_offset":10648,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11496,"parent_record_offset":11460,"ancestor_record_offset":10648,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11460,"parent_record_offset":11144,"ancestor_record_offset":10648,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11556,"parent_record_offset":11524,"ancestor_record_offset":10648,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11216,"parent_record_offset":11144,"ancestor_record_offset":10648,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11352,"parent_record_offset":11244,"ancestor_record_offset":10648,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":10648,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":12036,"parent_record_offset":11996,"ancestor_record_offset":10648,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":12076,"parent_record_offset":12036,"ancestor_record_offset":10648,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":12184,"parent_record_offset":11996,"ancestor_record_offset":10648,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":456,"record_offset":11996,"parent_record_offset":11948,"ancestor_record_offset":10648,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":10648,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":10908,"parent_record_offset":10852,"ancestor_record_offset":10648,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":456,"record_offset":12276,"parent_record_offset":11948,"ancestor_record_offset":10648,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":10648,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":456,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":10648,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":12376,"parent_record_offset":11948,"ancestor_record_offset":10648,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":10824,"parent_record_offset":10764,"ancestor_record_offset":10648,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":11948,"parent_record_offset":11920,"ancestor_record_offset":10648,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":10648,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":10648,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":10648,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":456,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":10648,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":456,"record_offset":11872,"parent_record_offset":11060,"ancestor_record_offset":10648,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":456,"record_offset":11388,"parent_record_offset":11352,"ancestor_record_offset":10648,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":456,"record_offset":11144,"parent_record_offset":11104,"ancestor_record_offset":10648,"inlinee":25667,"source_line":321,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11744,"parent_record_offset":11708,"ancestor_record_offset":10648,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":10648,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11668,"parent_record_offset":11616,"ancestor_record_offset":10648,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":10648,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":456,"record_offset":11708,"parent_record_offset":11144,"ancestor_record_offset":10648,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 321
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6443
// 首条位置: {"module":456,"inlinee":25667}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":456,"record_offset":11428,"parent_record_offset":11144,"ancestor_record_offset":11144,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11460,"parent_record_offset":11144,"ancestor_record_offset":11144,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11216,"parent_record_offset":11144,"ancestor_record_offset":11144,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":456,"record_offset":11708,"parent_record_offset":11144,"ancestor_record_offset":11144,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":456,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11144,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":456,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":11144,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":456,"record_offset":11292,"parent_record_offset":11244,"ancestor_record_offset":11144,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":456,"record_offset":11524,"parent_record_offset":11496,"ancestor_record_offset":11144,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11616,"parent_record_offset":11496,"ancestor_record_offset":11144,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11496,"parent_record_offset":11460,"ancestor_record_offset":11144,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":456,"record_offset":11556,"parent_record_offset":11524,"ancestor_record_offset":11144,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11352,"parent_record_offset":11244,"ancestor_record_offset":11144,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":456,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":11144,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":456,"record_offset":11388,"parent_record_offset":11352,"ancestor_record_offset":11144,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11744,"parent_record_offset":11708,"ancestor_record_offset":11144,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":456,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":11144,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11668,"parent_record_offset":11616,"ancestor_record_offset":11144,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00456.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":456,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":11144,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00456.jsonl.gz"}
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
// 完整结构及原始类型记录: derive_decorator_subtype.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: derive_decorator_subtype.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: derive_decorator_subtype.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::can_become_full_chimney
// 信息 ID: b1c494e906cf0dfe107a1569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=456, inline_record=10648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
// 信息 ID: 44ca5d78c2191a2c750238d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::can_become_full_chimney", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=456, inline_record=11144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype
// 信息 ID: 92749f1619b4c3228bd20a6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes @ 0x813149 -> 0x813d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x127d12e -> 0x813d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
// 信息 ID: 704b143f40ced96170248063
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=19048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
// 信息 ID: 23808965bac1d9169eb434f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=20588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
// 信息 ID: 130b0c5e49535d7e9b6f87c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=15844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
// 信息 ID: 5dce641d96cd1b25676d4d9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=17444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
// 信息 ID: fbfcbb8430a55c3195740c86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=18984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
// 信息 ID: f04bdec8e73d16fc6e6efab4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=456, inline_record=14204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtypes
// 信息 ID: 51d22f3080b38bae3cea2b4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 943e756bb07dafeded25746f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x3e914d -> 0x812b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > @ 0x450e64 -> 0x812b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure$0
// 信息 ID: 4278c53412f3a561e91f2074
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtypes", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; module=1546, inline_record=33328; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; module=1571, inline_record=9304; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; module=1571, inline_record=10728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::is_half_dormer
// 信息 ID: fa17eba84cbbc22a208aaf46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtype @ 0x8142b6 -> 0x814470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
