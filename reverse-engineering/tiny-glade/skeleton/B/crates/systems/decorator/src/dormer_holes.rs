// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/dormer_holes.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: eb7f0a97d8952707cd4b08d567d51bf5b3078e90ecbbcfef5ad71ed48ce5eab8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dormer_holes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 426
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::upload_dormer_holes_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":426,"record_offset":3168,"rva":8068416,"va":5376777536}
// 编译实例 RVAs: 0x7b1d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5861441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e841705900","size":5,"soft_fail":false,"target_va":5382644528,"va":5376783082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1697209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8471ae6ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376778692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1701648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8f008e6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376783147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1266880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e840abecff","size":5,"soft_fail":false,"target_va":5375516032,"va":5376782907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8fbc3ffff","size":5,"soft_fail":false,"target_va":5376776208,"va":5376791568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1918411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8cb451d00","size":5,"soft_fail":false,"target_va":5378702496,"va":5376784080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4438121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e869b84300","size":5,"soft_fail":false,"target_va":5381222304,"va":5376784178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141258610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11161532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8bc4faa00","size":5,"soft_fail":false,"target_va":5387945488,"va":5376783951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125ed60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11191091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e833c3aa00","size":5,"soft_fail":false,"target_va":5387971936,"va":5376780840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141261ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11199323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e85be3aa00","size":5,"soft_fail":false,"target_va":5387983776,"va":5376784448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35063126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e856051702","size":5,"soft_fail":false,"target_va":5411847152,"va":5376784021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35066270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e89e111702","size":5,"soft_fail":false,"target_va":5411847200,"va":5376780925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35065768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8a80f1702","size":5,"soft_fail":false,"target_va":5411847904,"va":5376782131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3992922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e85aed3c00","size":5,"soft_fail":false,"target_va":5380775408,"va":5376782481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34755103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e81f521202","size":5,"soft_fail":false,"target_va":5411540963,"va":5376785855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t34756260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8a4561202","size":5,"soft_fail":false,"target_va":5411541584,"va":5376785319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34696985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8196f1102","size":5,"soft_fail":false,"target_va":5411482433,"va":5376785443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34757033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8a9591202","size":5,"soft_fail":false,"target_va":5411542296,"va":5376785258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34756389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e825571202","size":5,"soft_fail":false,"target_va":5411542880,"va":5376786486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8d7faffff","size":5,"soft_fail":false,"target_va":5376776368,"va":5376777684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34765594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e81a7b1202","size":5,"soft_fail":false,"target_va":5411543360,"va":5376777761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::dummy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11355309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8ad44ad00","size":5,"soft_fail":false,"target_va":5388133984,"va":5376778670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4607347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8734d4600","size":5,"soft_fail":false,"target_va":5381385536,"va":5376778184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4612536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8b8614600","size":5,"soft_fail":false,"target_va":5381390640,"va":5376778099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4605376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8c0454600","size":5,"soft_fail":false,"target_va":5381386480,"va":5376781099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4607044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e8444c4600","size":5,"soft_fail":false,"target_va":5381384704,"va":5376777655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t34866257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e851041402","size":5,"soft_fail":false,"target_va":5411644800,"va":5376778538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11155538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e85238aa00","size":5,"soft_fail":false,"target_va":5387942848,"va":5376787305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t34751858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e872451202","size":5,"soft_fail":false,"target_va":5411537104,"va":5376785241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5146489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376777536,"raw_hex":"e879874e00","size":5,"soft_fail":false,"target_va":5381928576,"va":5376782082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376777536,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376784052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":7768,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14516,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":12684,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":14572,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":426,"record_offset":11940,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":426,"record_offset":19516,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":8496,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":426,"record_offset":23036,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":12944,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":426,"record_offset":18768,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":426,"record_offset":19008,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":25075,"source_line":357,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":4800,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":3376,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":3824,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":426,"record_offset":11580,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":25066,"source_line":311,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::get_cells_from_bbx
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":426,"record_offset":13004,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":25068,"source_line":135,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::upload_dormer_holes_to_gpu::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":426,"record_offset":14708,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":25074,"source_line":48,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":3168,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":3168,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":3168,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":7956,"parent_record_offset":7928,"ancestor_record_offset":3168,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":14440,"parent_record_offset":14348,"ancestor_record_offset":3168,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":3168,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":7868,"parent_record_offset":7840,"ancestor_record_offset":3168,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14376,"parent_record_offset":14348,"ancestor_record_offset":3168,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":3168,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":3168,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":3168,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":7928,"parent_record_offset":7896,"ancestor_record_offset":3168,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":7896,"parent_record_offset":7868,"ancestor_record_offset":3168,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":3168,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14132,"parent_record_offset":14092,"ancestor_record_offset":3168,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":3168,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":12740,"parent_record_offset":12708,"ancestor_record_offset":3168,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":8056,"parent_record_offset":7800,"ancestor_record_offset":3168,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":13780,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14092,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":7840,"parent_record_offset":7800,"ancestor_record_offset":3168,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14880,"parent_record_offset":14708,"ancestor_record_offset":3168,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14760,"parent_record_offset":14708,"ancestor_record_offset":3168,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":3168,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":3168,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":3168,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":3168,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":426,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":3168,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":426,"record_offset":7800,"parent_record_offset":7768,"ancestor_record_offset":3168,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":3168,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":3168,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":3168,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":3168,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5108,"parent_record_offset":5084,"ancestor_record_offset":3168,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5280,"parent_record_offset":5108,"ancestor_record_offset":3168,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":426,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":3168,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13364,"parent_record_offset":13336,"ancestor_record_offset":3168,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13720,"parent_record_offset":13692,"ancestor_record_offset":3168,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":15156,"parent_record_offset":14708,"ancestor_record_offset":3168,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13912,"parent_record_offset":13836,"ancestor_record_offset":3168,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":18836,"parent_record_offset":18796,"ancestor_record_offset":3168,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":426,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":3168,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":426,"record_offset":11852,"parent_record_offset":11820,"ancestor_record_offset":3168,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":426,"record_offset":13836,"parent_record_offset":13808,"ancestor_record_offset":3168,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":426,"record_offset":13808,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":426,"record_offset":19564,"parent_record_offset":19540,"ancestor_record_offset":3168,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":3168,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":426,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":3168,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":14408,"parent_record_offset":14376,"ancestor_record_offset":3168,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":426,"record_offset":3524,"parent_record_offset":3492,"ancestor_record_offset":3168,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":426,"record_offset":12232,"parent_record_offset":12200,"ancestor_record_offset":3168,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":426,"record_offset":12200,"parent_record_offset":12168,"ancestor_record_offset":3168,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":3168,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13216,"parent_record_offset":13188,"ancestor_record_offset":3168,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":426,"record_offset":4952,"parent_record_offset":4908,"ancestor_record_offset":3168,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":426,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":3168,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":426,"record_offset":13876,"parent_record_offset":13836,"ancestor_record_offset":3168,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":426,"record_offset":12624,"parent_record_offset":11940,"ancestor_record_offset":3168,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":426,"record_offset":15244,"parent_record_offset":15212,"ancestor_record_offset":3168,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":426,"record_offset":15272,"parent_record_offset":15212,"ancestor_record_offset":3168,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3168,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":426,"record_offset":5740,"parent_record_offset":4908,"ancestor_record_offset":3168,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":5344,"parent_record_offset":5056,"ancestor_record_offset":3168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":3168,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":426,"record_offset":12492,"parent_record_offset":12468,"ancestor_record_offset":3168,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3168,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":14648,"parent_record_offset":14572,"ancestor_record_offset":3168,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3168,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":426,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":3168,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":3168,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":3168,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":426,"record_offset":19116,"parent_record_offset":19076,"ancestor_record_offset":3168,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":426,"record_offset":19904,"parent_record_offset":19600,"ancestor_record_offset":3168,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":426,"record_offset":18920,"parent_record_offset":18892,"ancestor_record_offset":3168,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":426,"record_offset":18948,"parent_record_offset":18920,"ancestor_record_offset":3168,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":426,"record_offset":19644,"parent_record_offset":19600,"ancestor_record_offset":3168,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14908,"parent_record_offset":14880,"ancestor_record_offset":3168,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14796,"parent_record_offset":14760,"ancestor_record_offset":3168,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":3168,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14832,"parent_record_offset":14796,"ancestor_record_offset":3168,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":426,"record_offset":12880,"parent_record_offset":12708,"ancestor_record_offset":3168,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3168,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3168,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":3168,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":426,"record_offset":4140,"parent_record_offset":4024,"ancestor_record_offset":3168,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":3168,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":426,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":3168,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":3992,"parent_record_offset":3960,"ancestor_record_offset":3168,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":426,"record_offset":3960,"parent_record_offset":3924,"ancestor_record_offset":3168,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3168,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3168,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3168,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":426,"record_offset":4464,"parent_record_offset":3924,"ancestor_record_offset":3168,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3464,"parent_record_offset":3432,"ancestor_record_offset":3168,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":3168,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3624,"parent_record_offset":3592,"ancestor_record_offset":3168,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3720,"parent_record_offset":3624,"ancestor_record_offset":3168,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":3168,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":18864,"parent_record_offset":18836,"ancestor_record_offset":3168,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":3168,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":426,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":3168,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":4024,"parent_record_offset":3992,"ancestor_record_offset":3168,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":4308,"parent_record_offset":4024,"ancestor_record_offset":3168,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":4280,"parent_record_offset":4024,"ancestor_record_offset":3168,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":426,"record_offset":4084,"parent_record_offset":4024,"ancestor_record_offset":3168,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":3168,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":19160,"parent_record_offset":19008,"ancestor_record_offset":3168,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":3168,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":3168,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":426,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":3168,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":426,"record_offset":19044,"parent_record_offset":19008,"ancestor_record_offset":3168,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":426,"record_offset":19076,"parent_record_offset":19044,"ancestor_record_offset":3168,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13188,"parent_record_offset":13068,"ancestor_record_offset":3168,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13100,"parent_record_offset":13068,"ancestor_record_offset":3168,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec2::IVec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec2.rs
//     evidence={"module":426,"record_offset":13336,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":25071,"source_line":186,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec2::IVec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec2.rs
//     evidence={"module":426,"record_offset":13692,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":25072,"source_line":174,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":426,"record_offset":12128,"parent_record_offset":12088,"ancestor_record_offset":3168,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":426,"record_offset":12168,"parent_record_offset":12128,"ancestor_record_offset":3168,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":426,"record_offset":12276,"parent_record_offset":12088,"ancestor_record_offset":3168,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":426,"record_offset":12088,"parent_record_offset":12040,"ancestor_record_offset":3168,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":426,"record_offset":11764,"parent_record_offset":11736,"ancestor_record_offset":3168,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":426,"record_offset":11820,"parent_record_offset":11764,"ancestor_record_offset":3168,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":426,"record_offset":12368,"parent_record_offset":12040,"ancestor_record_offset":3168,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":426,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":3168,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":426,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":3168,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":12468,"parent_record_offset":12040,"ancestor_record_offset":3168,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":11736,"parent_record_offset":11676,"ancestor_record_offset":3168,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":12040,"parent_record_offset":12012,"ancestor_record_offset":3168,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":11652,"parent_record_offset":11628,"ancestor_record_offset":3168,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":11708,"parent_record_offset":11676,"ancestor_record_offset":3168,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":11676,"parent_record_offset":11652,"ancestor_record_offset":3168,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":426,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":3168,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15300,"parent_record_offset":15212,"ancestor_record_offset":3168,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":426,"record_offset":15188,"parent_record_offset":14708,"ancestor_record_offset":3168,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":426,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":3168,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":3168,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15072,"parent_record_offset":14708,"ancestor_record_offset":3168,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":4836,"parent_record_offset":4800,"ancestor_record_offset":3168,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":8576,"parent_record_offset":8536,"ancestor_record_offset":3168,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":9144,"parent_record_offset":8872,"ancestor_record_offset":3168,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":426,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":3168,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":5056,"parent_record_offset":4908,"ancestor_record_offset":3168,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":426,"record_offset":5316,"parent_record_offset":5056,"ancestor_record_offset":3168,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":426,"record_offset":19748,"parent_record_offset":19704,"ancestor_record_offset":3168,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":3168,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":426,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":3168,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":426,"record_offset":13276,"parent_record_offset":13068,"ancestor_record_offset":3168,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":426,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":3168,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":4908,"parent_record_offset":4872,"ancestor_record_offset":3168,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":426,"record_offset":4872,"parent_record_offset":4836,"ancestor_record_offset":3168,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":426,"record_offset":13068,"parent_record_offset":13004,"ancestor_record_offset":3168,"inlinee":25069,"source_line":121,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::upload_dormer_holes_to_gpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61F2
// 首条位置: {"module":426,"inlinee":25074}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14880,"parent_record_offset":14708,"ancestor_record_offset":14708,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14760,"parent_record_offset":14708,"ancestor_record_offset":14708,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":15156,"parent_record_offset":14708,"ancestor_record_offset":14708,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":426,"record_offset":15188,"parent_record_offset":14708,"ancestor_record_offset":14708,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15072,"parent_record_offset":14708,"ancestor_record_offset":14708,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":426,"record_offset":15244,"parent_record_offset":15212,"ancestor_record_offset":14708,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":426,"record_offset":15272,"parent_record_offset":15212,"ancestor_record_offset":14708,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14908,"parent_record_offset":14880,"ancestor_record_offset":14708,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14796,"parent_record_offset":14760,"ancestor_record_offset":14708,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":14708,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":426,"record_offset":14832,"parent_record_offset":14796,"ancestor_record_offset":14708,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":14708,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":426,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":14708,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15300,"parent_record_offset":15212,"ancestor_record_offset":14708,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":426,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":14708,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":426,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":14708,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61ED
// 首条位置: {"module":426,"inlinee":25069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13188,"parent_record_offset":13068,"ancestor_record_offset":13068,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13100,"parent_record_offset":13068,"ancestor_record_offset":13068,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":426,"record_offset":13276,"parent_record_offset":13068,"ancestor_record_offset":13068,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":13068,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13216,"parent_record_offset":13188,"ancestor_record_offset":13068,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::get_cells_from_bbx
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61EC
// 首条位置: {"module":426,"inlinee":25068}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":13780,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14092,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":426,"record_offset":13808,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":14276,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] glam::i32::ivec2::IVec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec2.rs
//     evidence={"module":426,"record_offset":13336,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":25071,"source_line":186,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] glam::i32::ivec2::IVec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec2.rs
//     evidence={"module":426,"record_offset":13692,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":25072,"source_line":174,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":426,"record_offset":13068,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":25069,"source_line":121,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13004,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":426,"record_offset":14440,"parent_record_offset":14348,"ancestor_record_offset":13004,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":13004,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14376,"parent_record_offset":14348,"ancestor_record_offset":13004,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":13004,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13004,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13004,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":426,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":13004,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":426,"record_offset":14132,"parent_record_offset":14092,"ancestor_record_offset":13004,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13364,"parent_record_offset":13336,"ancestor_record_offset":13004,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13720,"parent_record_offset":13692,"ancestor_record_offset":13004,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":426,"record_offset":13912,"parent_record_offset":13836,"ancestor_record_offset":13004,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":426,"record_offset":13836,"parent_record_offset":13808,"ancestor_record_offset":13004,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":426,"record_offset":14408,"parent_record_offset":14376,"ancestor_record_offset":13004,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":13004,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":426,"record_offset":13216,"parent_record_offset":13188,"ancestor_record_offset":13004,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":426,"record_offset":13876,"parent_record_offset":13836,"ancestor_record_offset":13004,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":13004,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":426,"record_offset":14244,"parent_record_offset":14092,"ancestor_record_offset":13004,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13188,"parent_record_offset":13068,"ancestor_record_offset":13004,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":426,"record_offset":13100,"parent_record_offset":13068,"ancestor_record_offset":13004,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00426.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":426,"record_offset":13276,"parent_record_offset":13068,"ancestor_record_offset":13004,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00426.jsonl.gz"}
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
// 完整结构及原始类型记录: dormer_holes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dormer_holes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dormer_holes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::get_cells_from_bbx
// 信息 ID: 49d45fa48d12268955ce58c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::upload_dormer_holes_to_gpu; module=426, inline_record=13004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::upload_dormer_holes_to_gpu
// 信息 ID: 16455cbe8c3af044b78dd6cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fe5c36d41c30ea6ec29120c4: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x3e63bc -> 0x7b1d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x46c08e -> 0x7b1d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::upload_dormer_holes_to_gpu::closure$0
// 信息 ID: 295df23f4c30983b450f6e38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::dormer_holes::upload_dormer_holes_to_gpu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::upload_dormer_holes_to_gpu; module=426, inline_record=14708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
// 信息 ID: 8c406a483da13ebf5359638f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::upload_dormer_holes_to_gpu; module=426, inline_record=13068; 不是独立调用边。
// [PDB 内联宿主] system_decorator::dormer_holes::upload_dormer_holes_to_gpu; module=426, inline_record=13424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
