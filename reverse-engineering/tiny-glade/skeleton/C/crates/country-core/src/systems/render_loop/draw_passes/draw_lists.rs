// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a4d08c02d0c34dcc436bd2fe6c9b9e257e806d42d5d3210396a1ca3b44c92d21
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_lists.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 555, 1766
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":27580,"rva":11417424,"va":5380126544}
// 编译实例 RVAs: 0xae3750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..shader..QuantizedLineWidth$u20$as$u20$core..convert..From$LT$f32$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3489740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8cc3f3500","size":5,"soft_fail":false,"target_va":5383621696,"va":5380131951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4453822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8420abcff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380132521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f680
//     owner=unresolved_namespace; count=2
//     target_aliases=274 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10901560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8c8a759ff","size":5,"soft_fail":false,"target_va":5369230976,"va":5380132531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140991e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1389553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e80fcceaff","size":5,"soft_fail":false,"target_va":5378743840,"va":5380133388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a8000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1298776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8a82eecff","size":5,"soft_fail":false,"target_va":5378834432,"va":5380133203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12647147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8ebfac000","size":5,"soft_fail":false,"target_va":5392780768,"va":5380133616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36219269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e885a92802","size":5,"soft_fail":false,"target_va":5416351504,"va":5380132230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36252358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8c62a2902","size":5,"soft_fail":false,"target_va":5416385024,"va":5380132661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=21
//     evidence={"assembly":"\tcallq\t36251106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8e2252902","size":5,"soft_fail":false,"target_va":5416381792,"va":5380130681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10821107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8f31da500","size":5,"soft_fail":false,"target_va":5390948224,"va":5380127112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14439880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8c855dc00","size":5,"soft_fail":false,"target_va":5394566880,"va":5380126995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::update_archetypes_unsafe_world_cell::hdddef606df2656e1 (.llvm.15498315384911152783)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14389619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e87391db00","size":5,"soft_fail":false,"target_va":5394516672,"va":5380127048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::validate_world::panic_mismatched
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36447782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e826262c02","size":5,"soft_fail":false,"target_va":5416580432,"va":5380132645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36488771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e843c62c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5380132632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36488737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e821c62c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380132858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36489467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8fbc82c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380132736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36488935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8e7c62c02","size":5,"soft_fail":false,"target_va":5416621762,"va":5380132822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36489215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8ffc72c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5380133020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e874230000","size":5,"soft_fail":false,"target_va":5380135696,"va":5380126615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36488472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e818c52c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5380132899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t36255651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8a3372902","size":5,"soft_fail":false,"target_va":5416388496,"va":5380132840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3836834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8a28b3a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380126953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3845341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8ddac3a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380126878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3832318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e8fe793a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380132589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3836219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e83b893a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380126592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36486656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380126544,"raw_hex":"e800be2c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380132699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":50168,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":30072,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":50196,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":555,"record_offset":30624,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":555,"record_offset":30100,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":29998,"source_line":1092,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query_filtered
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":555,"record_offset":30044,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":29997,"source_line":1573,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":53048,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":555,"record_offset":52824,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":555,"record_offset":30596,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":53488,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":51948,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":50256,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":50104,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":53520,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":55068,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":55040,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":52936,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":53880,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":52500,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::drawable::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":555,"record_offset":53076,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":34697,"source_line":154,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_lists::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":555,"record_offset":53460,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":34699,"source_line":58,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":29236,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":27840,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":28224,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::impl$44::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":555,"record_offset":52992,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":32857,"source_line":78,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":54068,"parent_record_offset":54044,"ancestor_record_offset":27580,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":54092,"parent_record_offset":54068,"ancestor_record_offset":27580,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":54116,"parent_record_offset":54092,"ancestor_record_offset":27580,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":54044,"parent_record_offset":53952,"ancestor_record_offset":27580,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":51200,"parent_record_offset":51168,"ancestor_record_offset":27580,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":49004,"parent_record_offset":48980,"ancestor_record_offset":27580,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":51224,"parent_record_offset":51200,"ancestor_record_offset":27580,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":53980,"parent_record_offset":53952,"ancestor_record_offset":27580,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":53952,"parent_record_offset":53928,"ancestor_record_offset":27580,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":49052,"parent_record_offset":49028,"ancestor_record_offset":27580,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":49028,"parent_record_offset":49004,"ancestor_record_offset":27580,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":53928,"parent_record_offset":53904,"ancestor_record_offset":27580,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":50484,"parent_record_offset":50456,"ancestor_record_offset":27580,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":48980,"parent_record_offset":48956,"ancestor_record_offset":27580,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":48956,"parent_record_offset":48932,"ancestor_record_offset":27580,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":30960,"parent_record_offset":30936,"ancestor_record_offset":27580,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":30984,"parent_record_offset":30960,"ancestor_record_offset":27580,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":50220,"parent_record_offset":50196,"ancestor_record_offset":27580,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":48932,"parent_record_offset":48908,"ancestor_record_offset":27580,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":39256,"parent_record_offset":39232,"ancestor_record_offset":27580,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":48908,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":38960,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":39044,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":49160,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":555,"record_offset":38876,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49360,"parent_record_offset":49296,"ancestor_record_offset":27580,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$119::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49188,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":34693,"source_line":2520,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":31088,"parent_record_offset":31056,"ancestor_record_offset":27580,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":39476,"parent_record_offset":39172,"ancestor_record_offset":27580,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":31360,"parent_record_offset":31056,"ancestor_record_offset":27580,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49940,"parent_record_offset":49916,"ancestor_record_offset":27580,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":27580,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":39172,"parent_record_offset":39136,"ancestor_record_offset":27580,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":39232,"parent_record_offset":39204,"ancestor_record_offset":27580,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":31056,"parent_record_offset":31020,"ancestor_record_offset":27580,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":31116,"parent_record_offset":31088,"ancestor_record_offset":27580,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49212,"parent_record_offset":49188,"ancestor_record_offset":27580,"inlinee":18918,"source_line":2277,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":555,"record_offset":49916,"parent_record_offset":49892,"ancestor_record_offset":27580,"inlinee":28978,"source_line":2286,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":555,"record_offset":30556,"parent_record_offset":30532,"ancestor_record_offset":27580,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":555,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":27580,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":555,"record_offset":30128,"parent_record_offset":30100,"ancestor_record_offset":27580,"inlinee":34686,"source_line":300,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":555,"record_offset":30188,"parent_record_offset":30128,"ancestor_record_offset":27580,"inlinee":34687,"source_line":316,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_unchecked_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":555,"record_offset":30324,"parent_record_offset":30188,"ancestor_record_offset":27580,"inlinee":34688,"source_line":366,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":555,"record_offset":30160,"parent_record_offset":30128,"ancestor_record_offset":27580,"inlinee":9051,"source_line":487,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$12::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":555,"record_offset":39136,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":34691,"source_line":182,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$12::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":555,"record_offset":31020,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":34690,"source_line":195,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":555,"record_offset":31660,"parent_record_offset":31636,"ancestor_record_offset":27580,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":31188,"parent_record_offset":31164,"ancestor_record_offset":27580,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":31828,"parent_record_offset":31800,"ancestor_record_offset":27580,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":31164,"parent_record_offset":31140,"ancestor_record_offset":27580,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":27580,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":31484,"parent_record_offset":31460,"ancestor_record_offset":27580,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":31412,"parent_record_offset":31384,"ancestor_record_offset":27580,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":31384,"parent_record_offset":31360,"ancestor_record_offset":27580,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":31140,"parent_record_offset":31116,"ancestor_record_offset":27580,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":30936,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":30852,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":555,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":27580,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":555,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":27580,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":555,"record_offset":30532,"parent_record_offset":30100,"ancestor_record_offset":27580,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":555,"record_offset":30224,"parent_record_offset":30188,"ancestor_record_offset":27580,"inlinee":4097,"source_line":183,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":555,"record_offset":30376,"parent_record_offset":30352,"ancestor_record_offset":27580,"inlinee":17701,"source_line":2890,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":555,"record_offset":30432,"parent_record_offset":30408,"ancestor_record_offset":27580,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::identifier::impl$10::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/identifier.rs
//     evidence={"module":555,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":27580,"inlinee":4099,"source_line":12,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":30408,"parent_record_offset":30324,"ancestor_record_offset":27580,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":30352,"parent_record_offset":30324,"ancestor_record_offset":27580,"inlinee":17700,"source_line":327,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":555,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":27580,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":555,"record_offset":49236,"parent_record_offset":49212,"ancestor_record_offset":27580,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29564,"parent_record_offset":29536,"ancestor_record_offset":27580,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29592,"parent_record_offset":29564,"ancestor_record_offset":27580,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29512,"parent_record_offset":29484,"ancestor_record_offset":27580,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29536,"parent_record_offset":29512,"ancestor_record_offset":27580,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29652,"parent_record_offset":29536,"ancestor_record_offset":27580,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":555,"record_offset":29764,"parent_record_offset":29740,"ancestor_record_offset":27580,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":30264,"parent_record_offset":30224,"ancestor_record_offset":27580,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":555,"record_offset":29616,"parent_record_offset":29592,"ancestor_record_offset":27580,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":555,"record_offset":54012,"parent_record_offset":53980,"ancestor_record_offset":27580,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":555,"record_offset":53288,"parent_record_offset":53260,"ancestor_record_offset":27580,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":555,"record_offset":29384,"parent_record_offset":29344,"ancestor_record_offset":27580,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":31956,"parent_record_offset":31932,"ancestor_record_offset":27580,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":52904,"parent_record_offset":52824,"ancestor_record_offset":27580,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":31304,"parent_record_offset":31280,"ancestor_record_offset":27580,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":31460,"parent_record_offset":31436,"ancestor_record_offset":27580,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":53104,"parent_record_offset":53076,"ancestor_record_offset":27580,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":555,"record_offset":28172,"parent_record_offset":28144,"ancestor_record_offset":27580,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":555,"record_offset":30000,"parent_record_offset":29344,"ancestor_record_offset":27580,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":29716,"parent_record_offset":29484,"ancestor_record_offset":27580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":49432,"parent_record_offset":49408,"ancestor_record_offset":27580,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":53260,"parent_record_offset":53236,"ancestor_record_offset":27580,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":53316,"parent_record_offset":53260,"ancestor_record_offset":27580,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":53340,"parent_record_offset":53316,"ancestor_record_offset":27580,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":30820,"parent_record_offset":30760,"ancestor_record_offset":27580,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":53212,"parent_record_offset":53188,"ancestor_record_offset":27580,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27580,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":27580,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":53236,"parent_record_offset":53212,"ancestor_record_offset":27580,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":28828,"parent_record_offset":28800,"ancestor_record_offset":27580,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":50596,"parent_record_offset":50456,"ancestor_record_offset":27580,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":555,"record_offset":28928,"parent_record_offset":28900,"ancestor_record_offset":27580,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":28900,"parent_record_offset":28872,"ancestor_record_offset":27580,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":27580,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":49104,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":27580,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":31540,"parent_record_offset":31508,"ancestor_record_offset":27580,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":555,"record_offset":52524,"parent_record_offset":52500,"ancestor_record_offset":27580,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":28572,"parent_record_offset":28548,"ancestor_record_offset":27580,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":28596,"parent_record_offset":28572,"ancestor_record_offset":27580,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":28956,"parent_record_offset":28928,"ancestor_record_offset":27580,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":30760,"parent_record_offset":30648,"ancestor_record_offset":27580,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":28548,"parent_record_offset":28432,"ancestor_record_offset":27580,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":555,"record_offset":27968,"parent_record_offset":27944,"ancestor_record_offset":27580,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":555,"record_offset":28260,"parent_record_offset":28224,"ancestor_record_offset":27580,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":555,"record_offset":28400,"parent_record_offset":28368,"ancestor_record_offset":27580,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":555,"record_offset":28368,"parent_record_offset":28332,"ancestor_record_offset":27580,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":555,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":27580,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":555,"record_offset":28332,"parent_record_offset":28308,"ancestor_record_offset":27580,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":555,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":27580,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":555,"record_offset":28872,"parent_record_offset":28332,"ancestor_record_offset":27580,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":27944,"parent_record_offset":27912,"ancestor_record_offset":27580,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":27912,"parent_record_offset":27884,"ancestor_record_offset":27580,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":28068,"parent_record_offset":28036,"ancestor_record_offset":27580,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":28120,"parent_record_offset":28068,"ancestor_record_offset":27580,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":28036,"parent_record_offset":28008,"ancestor_record_offset":27580,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":27884,"parent_record_offset":27840,"ancestor_record_offset":27580,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":27580,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":555,"record_offset":28432,"parent_record_offset":28400,"ancestor_record_offset":27580,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":555,"record_offset":28716,"parent_record_offset":28432,"ancestor_record_offset":27580,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":555,"record_offset":28688,"parent_record_offset":28432,"ancestor_record_offset":27580,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":555,"record_offset":28492,"parent_record_offset":28432,"ancestor_record_offset":27580,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":53388,"parent_record_offset":53160,"ancestor_record_offset":27580,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":30456,"parent_record_offset":30432,"ancestor_record_offset":27580,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":53412,"parent_record_offset":53388,"ancestor_record_offset":27580,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":30480,"parent_record_offset":30456,"ancestor_record_offset":27580,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":555,"record_offset":50136,"parent_record_offset":50104,"ancestor_record_offset":27580,"inlinee":34695,"source_line":301,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":555,"record_offset":31508,"parent_record_offset":31484,"ancestor_record_offset":27580,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":29272,"parent_record_offset":29236,"ancestor_record_offset":27580,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":54236,"parent_record_offset":54204,"ancestor_record_offset":27580,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":54676,"parent_record_offset":54472,"ancestor_record_offset":27580,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":555,"record_offset":29412,"parent_record_offset":29384,"ancestor_record_offset":27580,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":29484,"parent_record_offset":29344,"ancestor_record_offset":27580,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":555,"record_offset":29688,"parent_record_offset":29484,"ancestor_record_offset":27580,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$43::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":555,"record_offset":53136,"parent_record_offset":53104,"ancestor_record_offset":27580,"inlinee":30000,"source_line":78,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":555,"record_offset":53188,"parent_record_offset":53160,"ancestor_record_offset":27580,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":555,"record_offset":53016,"parent_record_offset":52992,"ancestor_record_offset":27580,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":555,"record_offset":29440,"parent_record_offset":29412,"ancestor_record_offset":27580,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":555,"record_offset":29344,"parent_record_offset":29308,"ancestor_record_offset":27580,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":555,"record_offset":29308,"parent_record_offset":29272,"ancestor_record_offset":27580,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$4::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x878B
// 首条位置: {"module":555,"inlinee":34699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":14536,"rva":11414704,"va":5380123824}
// 编译实例 RVAs: 0xae2cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3339705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e8b9f53200","size":5,"soft_fail":false,"target_va":5383464240,"va":5380124530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6645296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e8d0999aff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380124331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141537910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10832307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e8b349a500","size":5,"soft_fail":false,"target_va":5390956816,"va":5380124504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10895414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e83640a600","size":5,"soft_fail":false,"target_va":5391020064,"va":5380124645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36497114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e8dae62c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380124481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36497518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e86ee82c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380124685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3395025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e8d1cd3300","size":5,"soft_fail":false,"target_va":5383519664,"va":5380124634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3397004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e88cd53300","size":5,"soft_fail":false,"target_va":5383521616,"va":5380124607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t15445640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380123824,"raw_hex":"e888aeeb00","size":5,"soft_fail":false,"target_va":5395570160,"va":5380124515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":14728,"parent_record_offset":14536,"ancestor_record_offset":14536,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":18744,"parent_record_offset":14536,"ancestor_record_offset":14536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":555,"record_offset":18772,"parent_record_offset":14536,"ancestor_record_offset":14536,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":555,"record_offset":17736,"parent_record_offset":14536,"ancestor_record_offset":14536,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":14536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":14536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":14536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":14536,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":15996,"parent_record_offset":15956,"ancestor_record_offset":14536,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":14536,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":555,"record_offset":14756,"parent_record_offset":14728,"ancestor_record_offset":14536,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":14536,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":14536,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":16240,"parent_record_offset":16208,"ancestor_record_offset":14536,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":16184,"parent_record_offset":15676,"ancestor_record_offset":14536,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":16208,"parent_record_offset":16184,"ancestor_record_offset":14536,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":15704,"parent_record_offset":15676,"ancestor_record_offset":14536,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":16268,"parent_record_offset":16208,"ancestor_record_offset":14536,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":15752,"parent_record_offset":15728,"ancestor_record_offset":14536,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":14536,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":15932,"parent_record_offset":15908,"ancestor_record_offset":14536,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":15676,"parent_record_offset":14728,"ancestor_record_offset":14536,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":555,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":14536,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":555,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":14536,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":15452,"parent_record_offset":15424,"ancestor_record_offset":14536,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":14536,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":555,"record_offset":14996,"parent_record_offset":14972,"ancestor_record_offset":14536,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":555,"record_offset":15080,"parent_record_offset":15056,"ancestor_record_offset":14536,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":18568,"parent_record_offset":17904,"ancestor_record_offset":14536,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":555,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":14536,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":555,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":14536,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":555,"record_offset":15844,"parent_record_offset":15780,"ancestor_record_offset":14536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":14536,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":15908,"parent_record_offset":15728,"ancestor_record_offset":14536,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":14536,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":14536,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":15780,"parent_record_offset":15752,"ancestor_record_offset":14536,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":15816,"parent_record_offset":15780,"ancestor_record_offset":14536,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":16136,"parent_record_offset":15956,"ancestor_record_offset":14536,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":555,"record_offset":18520,"parent_record_offset":18496,"ancestor_record_offset":14536,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":555,"record_offset":18496,"parent_record_offset":18472,"ancestor_record_offset":14536,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":555,"record_offset":18808,"parent_record_offset":18772,"ancestor_record_offset":14536,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":555,"record_offset":19232,"parent_record_offset":18772,"ancestor_record_offset":14536,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":555,"record_offset":18868,"parent_record_offset":18836,"ancestor_record_offset":14536,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":555,"record_offset":18836,"parent_record_offset":18772,"ancestor_record_offset":14536,"inlinee":34678,"source_line":90,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":555,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":14536,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":15140,"parent_record_offset":15112,"ancestor_record_offset":14536,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":14536,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":15112,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":14972,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":15528,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":15056,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":555,"record_offset":17960,"parent_record_offset":17904,"ancestor_record_offset":14536,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":555,"record_offset":18448,"parent_record_offset":18424,"ancestor_record_offset":14536,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":555,"record_offset":14816,"parent_record_offset":14780,"ancestor_record_offset":14536,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":15376,"parent_record_offset":15272,"ancestor_record_offset":14536,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":17824,"parent_record_offset":17792,"ancestor_record_offset":14536,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":14536,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":15620,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":15292,"parent_record_offset":15272,"ancestor_record_offset":14536,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14536,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":14844,"parent_record_offset":14816,"ancestor_record_offset":14536,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":14892,"parent_record_offset":14868,"ancestor_record_offset":14536,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":14944,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":15272,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":555,"record_offset":15500,"parent_record_offset":14892,"ancestor_record_offset":14536,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":555,"record_offset":17556,"parent_record_offset":17532,"ancestor_record_offset":14536,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":14536,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":18908,"parent_record_offset":18868,"ancestor_record_offset":14536,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":14536,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":19096,"parent_record_offset":19068,"ancestor_record_offset":14536,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":18936,"parent_record_offset":18908,"ancestor_record_offset":14536,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":14536,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":555,"record_offset":18964,"parent_record_offset":18936,"ancestor_record_offset":14536,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":555,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":14536,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8776
// 首条位置: {"module":555,"inlinee":34678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":555,"record_offset":18868,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":18836,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":18908,"parent_record_offset":18868,"ancestor_record_offset":18836,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":18836,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":19096,"parent_record_offset":19068,"ancestor_record_offset":18836,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":18936,"parent_record_offset":18908,"ancestor_record_offset":18836,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":18836,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":555,"record_offset":18964,"parent_record_offset":18936,"ancestor_record_offset":18836,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::multi_instanced_indexed::draw_fn<country_core::resources::sparkle::SparkleSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":37780,"rva":28122416,"va":5396831536}
// 编译实例 RVAs: 0x1ad1d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13368000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e8400534ff","size":5,"soft_fail":false,"target_va":5383464240,"va":5396832235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21153762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e81e38bdfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396832461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23353024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e840a99bfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396832059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15254587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e8c53b17ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5396832326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a40470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-596995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e8fde3f6ff","size":5,"soft_fail":false,"target_va":5396235376,"va":5396832366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19789390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e84ef62d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396832205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19789702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e886f72d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5396832501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13310752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e8e0e434ff","size":5,"soft_fail":false,"target_va":5383521680,"va":5396832427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13310700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e814e534ff","size":5,"soft_fail":false,"target_va":5383521616,"va":5396832311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-1262065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396831536,"raw_hex":"e80fbeecff","size":5,"soft_fail":false,"target_va":5395570160,"va":5396832220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396831536,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396832347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":42596,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1766,"record_offset":42680,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":38040,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":42056,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":42828,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":42500,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1766,"record_offset":41048,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1766,"record_offset":42084,"parent_record_offset":37780,"ancestor_record_offset":37780,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1766,"record_offset":43084,"parent_record_offset":43060,"ancestor_record_offset":37780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1766,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":37780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1766,"record_offset":43132,"parent_record_offset":43108,"ancestor_record_offset":37780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1766,"record_offset":43060,"parent_record_offset":42972,"ancestor_record_offset":37780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":39356,"parent_record_offset":39332,"ancestor_record_offset":37780,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":43000,"parent_record_offset":42972,"ancestor_record_offset":37780,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":42972,"parent_record_offset":42948,"ancestor_record_offset":37780,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":39404,"parent_record_offset":39380,"ancestor_record_offset":37780,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":39380,"parent_record_offset":39356,"ancestor_record_offset":37780,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":42948,"parent_record_offset":42924,"ancestor_record_offset":37780,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":39332,"parent_record_offset":39308,"ancestor_record_offset":37780,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":39308,"parent_record_offset":39268,"ancestor_record_offset":37780,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":39268,"parent_record_offset":39244,"ancestor_record_offset":37780,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1766,"record_offset":42852,"parent_record_offset":42828,"ancestor_record_offset":37780,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1766,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":37780,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1766,"record_offset":38068,"parent_record_offset":38040,"ancestor_record_offset":37780,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":39628,"parent_record_offset":39604,"ancestor_record_offset":37780,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":39604,"parent_record_offset":39580,"ancestor_record_offset":37780,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":39552,"parent_record_offset":39520,"ancestor_record_offset":37780,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":39496,"parent_record_offset":38988,"ancestor_record_offset":37780,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":39520,"parent_record_offset":39496,"ancestor_record_offset":37780,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":39016,"parent_record_offset":38988,"ancestor_record_offset":37780,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":39580,"parent_record_offset":39520,"ancestor_record_offset":37780,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":39064,"parent_record_offset":39040,"ancestor_record_offset":37780,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":37780,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":39244,"parent_record_offset":39220,"ancestor_record_offset":37780,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":38988,"parent_record_offset":38040,"ancestor_record_offset":37780,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1766,"record_offset":38092,"parent_record_offset":38068,"ancestor_record_offset":37780,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1766,"record_offset":38736,"parent_record_offset":38712,"ancestor_record_offset":37780,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":38764,"parent_record_offset":38736,"ancestor_record_offset":37780,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":41784,"parent_record_offset":41760,"ancestor_record_offset":37780,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":38308,"parent_record_offset":38284,"ancestor_record_offset":37780,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":38392,"parent_record_offset":38368,"ancestor_record_offset":37780,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":41880,"parent_record_offset":41216,"ancestor_record_offset":37780,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1766,"record_offset":42620,"parent_record_offset":42596,"ancestor_record_offset":37780,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1766,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":37780,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1766,"record_offset":43028,"parent_record_offset":43000,"ancestor_record_offset":37780,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":38540,"parent_record_offset":38512,"ancestor_record_offset":37780,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":38512,"parent_record_offset":38484,"ancestor_record_offset":37780,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1766,"record_offset":39156,"parent_record_offset":39092,"ancestor_record_offset":37780,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":39180,"parent_record_offset":39156,"ancestor_record_offset":37780,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":39220,"parent_record_offset":39040,"ancestor_record_offset":37780,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":38332,"parent_record_offset":38308,"ancestor_record_offset":37780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":42876,"parent_record_offset":42852,"ancestor_record_offset":37780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1766,"record_offset":38652,"parent_record_offset":38628,"ancestor_record_offset":37780,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1766,"record_offset":42740,"parent_record_offset":42680,"ancestor_record_offset":37780,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1766,"record_offset":42768,"parent_record_offset":42680,"ancestor_record_offset":37780,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1766,"record_offset":42712,"parent_record_offset":42680,"ancestor_record_offset":37780,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":42792,"parent_record_offset":42768,"ancestor_record_offset":37780,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1766,"record_offset":39092,"parent_record_offset":39064,"ancestor_record_offset":37780,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":39128,"parent_record_offset":39092,"ancestor_record_offset":37780,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":39448,"parent_record_offset":39268,"ancestor_record_offset":37780,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":37780,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":37780,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":42524,"parent_record_offset":42500,"ancestor_record_offset":37780,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1766,"record_offset":38712,"parent_record_offset":38688,"ancestor_record_offset":37780,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":38452,"parent_record_offset":38424,"ancestor_record_offset":37780,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":38484,"parent_record_offset":38452,"ancestor_record_offset":37780,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":38424,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":38284,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":38840,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":38368,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1766,"record_offset":41272,"parent_record_offset":41216,"ancestor_record_offset":37780,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1766,"record_offset":41760,"parent_record_offset":41736,"ancestor_record_offset":37780,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":38128,"parent_record_offset":38092,"ancestor_record_offset":37780,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":38688,"parent_record_offset":38584,"ancestor_record_offset":37780,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":41136,"parent_record_offset":41104,"ancestor_record_offset":37780,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38628,"parent_record_offset":38604,"ancestor_record_offset":37780,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38932,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38604,"parent_record_offset":38584,"ancestor_record_offset":37780,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38180,"parent_record_offset":38156,"ancestor_record_offset":37780,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38156,"parent_record_offset":38128,"ancestor_record_offset":37780,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":37780,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38256,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":38584,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1766,"record_offset":38812,"parent_record_offset":38204,"ancestor_record_offset":37780,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1766,"record_offset":40868,"parent_record_offset":40844,"ancestor_record_offset":37780,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":42284,"parent_record_offset":42084,"ancestor_record_offset":37780,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":42124,"parent_record_offset":42084,"ancestor_record_offset":37780,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":42208,"parent_record_offset":42180,"ancestor_record_offset":37780,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":42312,"parent_record_offset":42284,"ancestor_record_offset":37780,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":42152,"parent_record_offset":42124,"ancestor_record_offset":37780,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":42236,"parent_record_offset":42208,"ancestor_record_offset":37780,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1766,"record_offset":42180,"parent_record_offset":42152,"ancestor_record_offset":37780,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1766,"record_offset":41076,"parent_record_offset":41048,"ancestor_record_offset":37780,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roof_tile_ssbo::RoofSpireSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":22208,"rva":28119584,"va":5396828704}
// 编译实例 RVAs: 0x1ad1220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13365259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8f50f34ff","size":5,"soft_fail":false,"target_va":5383464240,"va":5396829494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23350295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8e9b39bfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396829330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15251253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8cb4817ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5396828992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19792382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8fe012e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396829213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19792578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8c2022e01","size":5,"soft_fail":false,"target_va":5416622208,"va":5396829625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13307913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8f7ef34ff","size":5,"soft_fail":false,"target_va":5383521680,"va":5396829588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13307959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8c9ef34ff","size":5,"soft_fail":false,"target_va":5383521616,"va":5396829570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-1259324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396828704,"raw_hex":"e8c4c8ecff","size":5,"soft_fail":false,"target_va":5395570160,"va":5396829479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396828704,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396829013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":22464,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":24164,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":24104,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1766,"record_offset":25980,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1766,"record_offset":24200,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1766,"record_offset":26972,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":23780,"parent_record_offset":23756,"ancestor_record_offset":22208,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":23828,"parent_record_offset":23804,"ancestor_record_offset":22208,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":22208,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":22208,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":23732,"parent_record_offset":23692,"ancestor_record_offset":22208,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":22208,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1766,"record_offset":22492,"parent_record_offset":22464,"ancestor_record_offset":22208,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":24052,"parent_record_offset":24028,"ancestor_record_offset":22208,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":24028,"parent_record_offset":24004,"ancestor_record_offset":22208,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":23976,"parent_record_offset":23944,"ancestor_record_offset":22208,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":23920,"parent_record_offset":23412,"ancestor_record_offset":22208,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":22208,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":23440,"parent_record_offset":23412,"ancestor_record_offset":22208,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":24004,"parent_record_offset":23944,"ancestor_record_offset":22208,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":23488,"parent_record_offset":23464,"ancestor_record_offset":22208,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":23464,"parent_record_offset":23440,"ancestor_record_offset":22208,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":22208,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":23412,"parent_record_offset":22464,"ancestor_record_offset":22208,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1766,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":22208,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1766,"record_offset":23160,"parent_record_offset":23136,"ancestor_record_offset":22208,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":23188,"parent_record_offset":23160,"ancestor_record_offset":22208,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":26700,"parent_record_offset":26676,"ancestor_record_offset":22208,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":22732,"parent_record_offset":22708,"ancestor_record_offset":22208,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22208,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":26796,"parent_record_offset":26132,"ancestor_record_offset":22208,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":22964,"parent_record_offset":22936,"ancestor_record_offset":22208,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":22936,"parent_record_offset":22908,"ancestor_record_offset":22208,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1766,"record_offset":23580,"parent_record_offset":23516,"ancestor_record_offset":22208,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":23604,"parent_record_offset":23580,"ancestor_record_offset":22208,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":23644,"parent_record_offset":23464,"ancestor_record_offset":22208,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":22756,"parent_record_offset":22732,"ancestor_record_offset":22208,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1766,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22208,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1766,"record_offset":23516,"parent_record_offset":23488,"ancestor_record_offset":22208,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":23552,"parent_record_offset":23516,"ancestor_record_offset":22208,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":23872,"parent_record_offset":23692,"ancestor_record_offset":22208,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":22208,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":22208,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":26748,"parent_record_offset":26724,"ancestor_record_offset":22208,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":26724,"parent_record_offset":26700,"ancestor_record_offset":22208,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":22208,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1766,"record_offset":23136,"parent_record_offset":23112,"ancestor_record_offset":22208,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":22876,"parent_record_offset":22848,"ancestor_record_offset":22208,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":22908,"parent_record_offset":22876,"ancestor_record_offset":22208,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":22848,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":22708,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":23264,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":22792,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1766,"record_offset":26188,"parent_record_offset":26132,"ancestor_record_offset":22208,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1766,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":22208,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":22552,"parent_record_offset":22516,"ancestor_record_offset":22208,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":23112,"parent_record_offset":23008,"ancestor_record_offset":22208,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":26056,"parent_record_offset":26028,"ancestor_record_offset":22208,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22208,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":23356,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":23028,"parent_record_offset":23008,"ancestor_record_offset":22208,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":22604,"parent_record_offset":22580,"ancestor_record_offset":22208,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":22580,"parent_record_offset":22552,"ancestor_record_offset":22208,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":22628,"parent_record_offset":22604,"ancestor_record_offset":22208,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":22680,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":23008,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1766,"record_offset":23236,"parent_record_offset":22628,"ancestor_record_offset":22208,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":27172,"parent_record_offset":26972,"ancestor_record_offset":22208,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":27012,"parent_record_offset":26972,"ancestor_record_offset":22208,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":27096,"parent_record_offset":27068,"ancestor_record_offset":22208,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":27200,"parent_record_offset":27172,"ancestor_record_offset":22208,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":27040,"parent_record_offset":27012,"ancestor_record_offset":22208,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":27124,"parent_record_offset":27096,"ancestor_record_offset":22208,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1766,"record_offset":27068,"parent_record_offset":27040,"ancestor_record_offset":22208,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1766,"record_offset":26004,"parent_record_offset":25980,"ancestor_record_offset":22208,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roofs::roof_buffers::RoofSupportBeamSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":32584,"rva":28121472,"va":5396830592}
// 编译实例 RVAs: 0x1ad1980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13367147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e8950834ff","size":5,"soft_fail":false,"target_va":5383464240,"va":5396831382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23352183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e889ac9bfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396831218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15253141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e86b4117ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5396830880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19790494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e89efa2d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396831101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19790690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e862fb2d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5396831513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13309801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e897e834ff","size":5,"soft_fail":false,"target_va":5383521680,"va":5396831476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13309847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e869e834ff","size":5,"soft_fail":false,"target_va":5383521616,"va":5396831458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-1261212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396830592,"raw_hex":"e864c1ecff","size":5,"soft_fail":false,"target_va":5395570160,"va":5396831367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396830592,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396830901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":32852,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":34552,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":34492,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1766,"record_offset":36368,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1766,"record_offset":34588,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1766,"record_offset":37360,"parent_record_offset":32584,"ancestor_record_offset":32584,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":34168,"parent_record_offset":34144,"ancestor_record_offset":32584,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":34216,"parent_record_offset":34192,"ancestor_record_offset":32584,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":34192,"parent_record_offset":34168,"ancestor_record_offset":32584,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":34144,"parent_record_offset":34120,"ancestor_record_offset":32584,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":34120,"parent_record_offset":34080,"ancestor_record_offset":32584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":32584,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1766,"record_offset":32880,"parent_record_offset":32852,"ancestor_record_offset":32584,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":34440,"parent_record_offset":34416,"ancestor_record_offset":32584,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":34416,"parent_record_offset":34392,"ancestor_record_offset":32584,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":34364,"parent_record_offset":34332,"ancestor_record_offset":32584,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":34308,"parent_record_offset":33800,"ancestor_record_offset":32584,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":34332,"parent_record_offset":34308,"ancestor_record_offset":32584,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":33828,"parent_record_offset":33800,"ancestor_record_offset":32584,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":34392,"parent_record_offset":34332,"ancestor_record_offset":32584,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":33876,"parent_record_offset":33852,"ancestor_record_offset":32584,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":33852,"parent_record_offset":33828,"ancestor_record_offset":32584,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":34056,"parent_record_offset":34032,"ancestor_record_offset":32584,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":33800,"parent_record_offset":32852,"ancestor_record_offset":32584,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1766,"record_offset":32904,"parent_record_offset":32880,"ancestor_record_offset":32584,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1766,"record_offset":33548,"parent_record_offset":33524,"ancestor_record_offset":32584,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":33576,"parent_record_offset":33548,"ancestor_record_offset":32584,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":37088,"parent_record_offset":37064,"ancestor_record_offset":32584,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":33120,"parent_record_offset":33096,"ancestor_record_offset":32584,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":33204,"parent_record_offset":33180,"ancestor_record_offset":32584,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":37184,"parent_record_offset":36520,"ancestor_record_offset":32584,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":33352,"parent_record_offset":33324,"ancestor_record_offset":32584,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":33324,"parent_record_offset":33296,"ancestor_record_offset":32584,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1766,"record_offset":33968,"parent_record_offset":33904,"ancestor_record_offset":32584,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":33992,"parent_record_offset":33968,"ancestor_record_offset":32584,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":34032,"parent_record_offset":33852,"ancestor_record_offset":32584,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":33144,"parent_record_offset":33120,"ancestor_record_offset":32584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1766,"record_offset":33464,"parent_record_offset":33440,"ancestor_record_offset":32584,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1766,"record_offset":33904,"parent_record_offset":33876,"ancestor_record_offset":32584,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":33940,"parent_record_offset":33904,"ancestor_record_offset":32584,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":34260,"parent_record_offset":34080,"ancestor_record_offset":32584,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":34612,"parent_record_offset":34588,"ancestor_record_offset":32584,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":34636,"parent_record_offset":34612,"ancestor_record_offset":32584,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":37136,"parent_record_offset":37112,"ancestor_record_offset":32584,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":37112,"parent_record_offset":37088,"ancestor_record_offset":32584,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":34516,"parent_record_offset":34492,"ancestor_record_offset":32584,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1766,"record_offset":33524,"parent_record_offset":33500,"ancestor_record_offset":32584,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":33264,"parent_record_offset":33236,"ancestor_record_offset":32584,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":33296,"parent_record_offset":33264,"ancestor_record_offset":32584,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":33236,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":33096,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":33652,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":33180,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1766,"record_offset":36576,"parent_record_offset":36520,"ancestor_record_offset":32584,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1766,"record_offset":37064,"parent_record_offset":37040,"ancestor_record_offset":32584,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":32940,"parent_record_offset":32904,"ancestor_record_offset":32584,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":33500,"parent_record_offset":33396,"ancestor_record_offset":32584,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":36444,"parent_record_offset":36416,"ancestor_record_offset":32584,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33440,"parent_record_offset":33416,"ancestor_record_offset":32584,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33744,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33416,"parent_record_offset":33396,"ancestor_record_offset":32584,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":32584,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":32968,"parent_record_offset":32940,"ancestor_record_offset":32584,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":32584,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33068,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":33396,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1766,"record_offset":33624,"parent_record_offset":33016,"ancestor_record_offset":32584,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":37560,"parent_record_offset":37360,"ancestor_record_offset":32584,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":37400,"parent_record_offset":37360,"ancestor_record_offset":32584,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":37484,"parent_record_offset":37456,"ancestor_record_offset":32584,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":37588,"parent_record_offset":37560,"ancestor_record_offset":32584,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":37428,"parent_record_offset":37400,"ancestor_record_offset":32584,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":37512,"parent_record_offset":37484,"ancestor_record_offset":32584,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1766,"record_offset":37456,"parent_record_offset":37428,"ancestor_record_offset":32584,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1766,"record_offset":36392,"parent_record_offset":36368,"ancestor_record_offset":32584,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":17016,"rva":28118640,"va":5396827760}
// 编译实例 RVAs: 0x1ad0e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13364315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e8a51334ff","size":5,"soft_fail":false,"target_va":5383464240,"va":5396828550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23349351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e899b79bfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396828386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15250309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e87b4c17ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5396828048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19793326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e8ae052e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396828269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19793522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e872062e01","size":5,"soft_fail":false,"target_va":5416622208,"va":5396828681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13306969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e8a7f334ff","size":5,"soft_fail":false,"target_va":5383521680,"va":5396828644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13307015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e879f334ff","size":5,"soft_fail":false,"target_va":5383521616,"va":5396828626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-1258380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396827760,"raw_hex":"e874ccecff","size":5,"soft_fail":false,"target_va":5395570160,"va":5396828535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396827760,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396828069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":17280,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":18980,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":18920,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1766,"record_offset":20796,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1766,"record_offset":19016,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1766,"record_offset":21788,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":17016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":17016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":17016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":18572,"parent_record_offset":18548,"ancestor_record_offset":17016,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":18548,"parent_record_offset":18508,"ancestor_record_offset":17016,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":17016,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1766,"record_offset":17308,"parent_record_offset":17280,"ancestor_record_offset":17016,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":17016,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":17016,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":18792,"parent_record_offset":18760,"ancestor_record_offset":17016,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":18736,"parent_record_offset":18228,"ancestor_record_offset":17016,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":17016,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":18256,"parent_record_offset":18228,"ancestor_record_offset":17016,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":18820,"parent_record_offset":18760,"ancestor_record_offset":17016,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":17016,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":17016,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":18484,"parent_record_offset":18460,"ancestor_record_offset":17016,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":18228,"parent_record_offset":17280,"ancestor_record_offset":17016,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1766,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":17016,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1766,"record_offset":17976,"parent_record_offset":17952,"ancestor_record_offset":17016,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":18004,"parent_record_offset":17976,"ancestor_record_offset":17016,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":21516,"parent_record_offset":21492,"ancestor_record_offset":17016,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":17016,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":17016,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":21612,"parent_record_offset":20948,"ancestor_record_offset":17016,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":17780,"parent_record_offset":17752,"ancestor_record_offset":17016,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":17752,"parent_record_offset":17724,"ancestor_record_offset":17016,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1766,"record_offset":18396,"parent_record_offset":18332,"ancestor_record_offset":17016,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":17016,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":18460,"parent_record_offset":18280,"ancestor_record_offset":17016,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":17572,"parent_record_offset":17548,"ancestor_record_offset":17016,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1766,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":17016,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1766,"record_offset":18332,"parent_record_offset":18304,"ancestor_record_offset":17016,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":18368,"parent_record_offset":18332,"ancestor_record_offset":17016,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":18688,"parent_record_offset":18508,"ancestor_record_offset":17016,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":17016,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":19064,"parent_record_offset":19040,"ancestor_record_offset":17016,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":21564,"parent_record_offset":21540,"ancestor_record_offset":17016,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":17016,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":18944,"parent_record_offset":18920,"ancestor_record_offset":17016,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1766,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":17016,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":17016,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":17724,"parent_record_offset":17692,"ancestor_record_offset":17016,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":17664,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":17524,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":18080,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":17608,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1766,"record_offset":21004,"parent_record_offset":20948,"ancestor_record_offset":17016,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1766,"record_offset":21492,"parent_record_offset":21468,"ancestor_record_offset":17016,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":17368,"parent_record_offset":17332,"ancestor_record_offset":17016,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":17928,"parent_record_offset":17824,"ancestor_record_offset":17016,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":20872,"parent_record_offset":20844,"ancestor_record_offset":17016,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":17016,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":18172,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17844,"parent_record_offset":17824,"ancestor_record_offset":17016,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":17016,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17396,"parent_record_offset":17368,"ancestor_record_offset":17016,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17444,"parent_record_offset":17420,"ancestor_record_offset":17016,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17496,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":17824,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1766,"record_offset":18052,"parent_record_offset":17444,"ancestor_record_offset":17016,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":21988,"parent_record_offset":21788,"ancestor_record_offset":17016,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":21828,"parent_record_offset":21788,"ancestor_record_offset":17016,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":21912,"parent_record_offset":21884,"ancestor_record_offset":17016,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":22016,"parent_record_offset":21988,"ancestor_record_offset":17016,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":21856,"parent_record_offset":21828,"ancestor_record_offset":17016,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":21940,"parent_record_offset":21912,"ancestor_record_offset":17016,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1766,"record_offset":21884,"parent_record_offset":21856,"ancestor_record_offset":17016,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1766,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":17016,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1766,"record_offset":27392,"rva":28120528,"va":5396829648}
// 编译实例 RVAs: 0x1ad15d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13366203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e8450c34ff","size":5,"soft_fail":false,"target_va":5383464240,"va":5396830438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23351239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e839b09bfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396830274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15252197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e81b4517ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5396829936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19791438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e84efe2d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396830157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19791634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e812ff2d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5396830569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13308857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e847ec34ff","size":5,"soft_fail":false,"target_va":5383521680,"va":5396830532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-13308903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e819ec34ff","size":5,"soft_fail":false,"target_va":5383521616,"va":5396830514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-1260268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396829648,"raw_hex":"e814c5ecff","size":5,"soft_fail":false,"target_va":5395570160,"va":5396830423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396829648,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396829957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":27656,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":29356,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":29296,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1766,"record_offset":31172,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1766,"record_offset":29392,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":1766,"record_offset":32164,"parent_record_offset":27392,"ancestor_record_offset":27392,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":28972,"parent_record_offset":28948,"ancestor_record_offset":27392,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":29020,"parent_record_offset":28996,"ancestor_record_offset":27392,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1766,"record_offset":28996,"parent_record_offset":28972,"ancestor_record_offset":27392,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":28948,"parent_record_offset":28924,"ancestor_record_offset":27392,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":28924,"parent_record_offset":28884,"ancestor_record_offset":27392,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1766,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":27392,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":1766,"record_offset":27684,"parent_record_offset":27656,"ancestor_record_offset":27392,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":29244,"parent_record_offset":29220,"ancestor_record_offset":27392,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":29220,"parent_record_offset":29196,"ancestor_record_offset":27392,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1766,"record_offset":29168,"parent_record_offset":29136,"ancestor_record_offset":27392,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":29112,"parent_record_offset":28604,"ancestor_record_offset":27392,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":27392,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":28632,"parent_record_offset":28604,"ancestor_record_offset":27392,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1766,"record_offset":29196,"parent_record_offset":29136,"ancestor_record_offset":27392,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":28680,"parent_record_offset":28656,"ancestor_record_offset":27392,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":28656,"parent_record_offset":28632,"ancestor_record_offset":27392,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1766,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":27392,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1766,"record_offset":28604,"parent_record_offset":27656,"ancestor_record_offset":27392,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":1766,"record_offset":27708,"parent_record_offset":27684,"ancestor_record_offset":27392,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1766,"record_offset":28352,"parent_record_offset":28328,"ancestor_record_offset":27392,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":28380,"parent_record_offset":28352,"ancestor_record_offset":27392,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1766,"record_offset":31892,"parent_record_offset":31868,"ancestor_record_offset":27392,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":27924,"parent_record_offset":27900,"ancestor_record_offset":27392,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1766,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":27392,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1766,"record_offset":31988,"parent_record_offset":31324,"ancestor_record_offset":27392,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":28156,"parent_record_offset":28128,"ancestor_record_offset":27392,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1766,"record_offset":28128,"parent_record_offset":28100,"ancestor_record_offset":27392,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1766,"record_offset":28772,"parent_record_offset":28708,"ancestor_record_offset":27392,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":28796,"parent_record_offset":28772,"ancestor_record_offset":27392,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1766,"record_offset":28836,"parent_record_offset":28656,"ancestor_record_offset":27392,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1766,"record_offset":27948,"parent_record_offset":27924,"ancestor_record_offset":27392,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1766,"record_offset":28268,"parent_record_offset":28244,"ancestor_record_offset":27392,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1766,"record_offset":28708,"parent_record_offset":28680,"ancestor_record_offset":27392,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":28744,"parent_record_offset":28708,"ancestor_record_offset":27392,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1766,"record_offset":29064,"parent_record_offset":28884,"ancestor_record_offset":27392,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":29416,"parent_record_offset":29392,"ancestor_record_offset":27392,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1766,"record_offset":29440,"parent_record_offset":29416,"ancestor_record_offset":27392,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":31940,"parent_record_offset":31916,"ancestor_record_offset":27392,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1766,"record_offset":31916,"parent_record_offset":31892,"ancestor_record_offset":27392,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1766,"record_offset":29320,"parent_record_offset":29296,"ancestor_record_offset":27392,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1766,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":27392,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":28068,"parent_record_offset":28040,"ancestor_record_offset":27392,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":28100,"parent_record_offset":28068,"ancestor_record_offset":27392,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1766,"record_offset":28040,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":27900,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":28456,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1766,"record_offset":27984,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1766,"record_offset":31380,"parent_record_offset":31324,"ancestor_record_offset":27392,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1766,"record_offset":31868,"parent_record_offset":31844,"ancestor_record_offset":27392,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":27744,"parent_record_offset":27708,"ancestor_record_offset":27392,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":28304,"parent_record_offset":28200,"ancestor_record_offset":27392,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1766,"record_offset":31248,"parent_record_offset":31220,"ancestor_record_offset":27392,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":28244,"parent_record_offset":28220,"ancestor_record_offset":27392,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":28548,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":28220,"parent_record_offset":28200,"ancestor_record_offset":27392,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":27796,"parent_record_offset":27772,"ancestor_record_offset":27392,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":27772,"parent_record_offset":27744,"ancestor_record_offset":27392,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":27820,"parent_record_offset":27796,"ancestor_record_offset":27392,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":27872,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1766,"record_offset":28200,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":1766,"record_offset":28428,"parent_record_offset":27820,"ancestor_record_offset":27392,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":32364,"parent_record_offset":32164,"ancestor_record_offset":27392,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":32204,"parent_record_offset":32164,"ancestor_record_offset":27392,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":32288,"parent_record_offset":32260,"ancestor_record_offset":27392,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":32392,"parent_record_offset":32364,"ancestor_record_offset":27392,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":1766,"record_offset":32232,"parent_record_offset":32204,"ancestor_record_offset":27392,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":1766,"record_offset":32316,"parent_record_offset":32288,"ancestor_record_offset":27392,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":1766,"record_offset":32260,"parent_record_offset":32232,"ancestor_record_offset":27392,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1766,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":27392,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::impl$15::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x878D
// 首条位置: {"module":555,"inlinee":34701}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$41::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":555,"record_offset":56264,"parent_record_offset":56240,"ancestor_record_offset":56240,"inlinee":34703,"source_line":493,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":56080,"rva":11424528,"va":5380133648}
// 编译实例 RVAs: 0xae5310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_lists::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":555,"record_offset":56240,"parent_record_offset":56080,"ancestor_record_offset":56080,"inlinee":34701,"source_line":227,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$41::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":555,"record_offset":56264,"parent_record_offset":56240,"ancestor_record_offset":56080,"inlinee":34703,"source_line":493,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
// 记录形式: compiled_procedure; instances=1
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":56300,"rva":11424912,"va":5380134032}
// 编译实例 RVAs: 0xae5490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14152d2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10780163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380134032,"raw_hex":"e8037ea400","size":5,"soft_fail":false,"target_va":5390914288,"va":5380134120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1740246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380134032,"raw_hex":"e8d68d1a00","size":5,"soft_fail":false,"target_va":5381874400,"va":5380134149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":56464,"parent_record_offset":56300,"ancestor_record_offset":56300,"inlinee":8431,"source_line":247,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":56632,"parent_record_offset":56300,"ancestor_record_offset":56300,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":56496,"parent_record_offset":56464,"ancestor_record_offset":56300,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":56564,"parent_record_offset":56528,"ancestor_record_offset":56300,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":56660,"parent_record_offset":56632,"ancestor_record_offset":56300,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":555,"record_offset":56528,"parent_record_offset":56496,"ancestor_record_offset":56300,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8787
// 首条位置: {"module":555,"inlinee":34695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":55876,"rva":47679376,"va":5416388496}
// 编译实例 RVAs: 0x2d78790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t233669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416388496,"raw_hex":"e8c5900300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416388566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::_$LT$impl$u20$core..ops..deref..Deref$u20$for$u20$rhapsody..raw..respool..Handle$LT$rhapsody..raw..shader..RasterPipelineSchema$GT$$GT$::deref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-32562029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416388496,"raw_hex":"e893240ffe","size":5,"soft_fail":false,"target_va":5383826480,"va":5416388504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::shader::ShaderPipelineCommon::debug_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":555,"record_offset":56048,"parent_record_offset":55876,"ancestor_record_offset":55876,"inlinee":10388,"source_line":150,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_lists::render_bucket
// 记录形式: compiled_procedure; instances=1
// 源码行: 479
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":19324,"rva":11415584,"va":5380124704}
// 编译实例 RVAs: 0xae3020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3337919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8bfee3200","size":5,"soft_fail":false,"target_va":5383464240,"va":5380126316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6647115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8b5929aff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380126150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6644627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e86d9c9aff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380125518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36931387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e83b873302","size":5,"soft_fail":false,"target_va":5417057040,"va":5380125648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t36495195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e85bdf2c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380126400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::_$LT$impl$u20$core..ops..deref..Deref$u20$for$u20$rhapsody..raw..respool..Handle$LT$rhapsody..raw..shader..RasterPipelineSchema$GT$$GT$::deref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3701010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e812793800","size":5,"soft_fail":false,"target_va":5383826480,"va":5380125465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::find_binding_set_stack_item
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3425211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8bb433400","size":5,"soft_fail":false,"target_va":5383550480,"va":5380125264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3380738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e802963300","size":5,"soft_fail":false,"target_va":5383506464,"va":5380125721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::bind_stack_sets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3394231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8b7ca3300","size":5,"soft_fail":false,"target_va":5383520000,"va":5380125764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3390529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e841bc3300","size":5,"soft_fail":false,"target_va":5383516320,"va":5380125786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3395787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8cbd03300","size":5,"soft_fail":false,"target_va":5383521680,"va":5380125888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3395233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e8a1ce3300","size":5,"soft_fail":false,"target_va":5383521616,"va":5380126378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t15443855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380124704,"raw_hex":"e88fa7eb00","size":5,"soft_fail":false,"target_va":5395570160,"va":5380126300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380124704,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5380125432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":19528,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":22716,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":22880,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":555,"record_offset":26144,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":555,"record_offset":27136,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":555,"record_offset":21172,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::QuantizedLineWidth::is_nonzero
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":555,"record_offset":24260,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":34682,"source_line":771,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::rg
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":22688,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":34679,"source_line":1266,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::line_width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":24344,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":34685,"source_line":2133,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::stencil_ref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":24104,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":34680,"source_line":2126,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":555,"record_offset":23028,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":555,"record_offset":22944,"parent_record_offset":19324,"ancestor_record_offset":19324,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":20848,"parent_record_offset":20824,"ancestor_record_offset":19324,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":19324,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":555,"record_offset":20872,"parent_record_offset":20848,"ancestor_record_offset":19324,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":20824,"parent_record_offset":20800,"ancestor_record_offset":19324,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":20800,"parent_record_offset":20760,"ancestor_record_offset":19324,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":19324,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":555,"record_offset":19556,"parent_record_offset":19528,"ancestor_record_offset":19324,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":21120,"parent_record_offset":21096,"ancestor_record_offset":19324,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":19324,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":555,"record_offset":21044,"parent_record_offset":21012,"ancestor_record_offset":19324,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":20988,"parent_record_offset":20480,"ancestor_record_offset":19324,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":19324,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":19324,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":555,"record_offset":21072,"parent_record_offset":21012,"ancestor_record_offset":19324,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":19324,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":20532,"parent_record_offset":20508,"ancestor_record_offset":19324,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":555,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":19324,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":21204,"parent_record_offset":21172,"ancestor_record_offset":19324,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":555,"record_offset":20480,"parent_record_offset":19528,"ancestor_record_offset":19324,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":555,"record_offset":19580,"parent_record_offset":19556,"ancestor_record_offset":19324,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":555,"record_offset":20228,"parent_record_offset":20204,"ancestor_record_offset":19324,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":20256,"parent_record_offset":20228,"ancestor_record_offset":19324,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":24284,"parent_record_offset":24260,"ancestor_record_offset":19324,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":23760,"parent_record_offset":23732,"ancestor_record_offset":19324,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":555,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19324,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":555,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":19324,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":23928,"parent_record_offset":23184,"ancestor_record_offset":19324,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":555,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":19324,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":555,"record_offset":20000,"parent_record_offset":19972,"ancestor_record_offset":19324,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":555,"record_offset":20648,"parent_record_offset":20584,"ancestor_record_offset":19324,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":20672,"parent_record_offset":20648,"ancestor_record_offset":19324,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":20712,"parent_record_offset":20532,"ancestor_record_offset":19324,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":555,"record_offset":22656,"parent_record_offset":21172,"ancestor_record_offset":19324,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":19324,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":19324,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":22908,"parent_record_offset":22880,"ancestor_record_offset":19324,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":555,"record_offset":23844,"parent_record_offset":23816,"ancestor_record_offset":19324,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":555,"record_offset":23872,"parent_record_offset":23844,"ancestor_record_offset":19324,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":555,"record_offset":20584,"parent_record_offset":20556,"ancestor_record_offset":19324,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":20620,"parent_record_offset":20584,"ancestor_record_offset":19324,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":555,"record_offset":20940,"parent_record_offset":20760,"ancestor_record_offset":19324,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":555,"record_offset":23788,"parent_record_offset":23760,"ancestor_record_offset":19324,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":555,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":19324,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":555,"record_offset":26888,"parent_record_offset":26864,"ancestor_record_offset":19324,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":555,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":19324,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":19940,"parent_record_offset":19912,"ancestor_record_offset":19324,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":19972,"parent_record_offset":19940,"ancestor_record_offset":19324,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":555,"record_offset":19912,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":19772,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":20332,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":555,"record_offset":19856,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":555,"record_offset":23240,"parent_record_offset":23184,"ancestor_record_offset":19324,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":555,"record_offset":23732,"parent_record_offset":23704,"ancestor_record_offset":19324,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":19324,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":555,"record_offset":19616,"parent_record_offset":19580,"ancestor_record_offset":19324,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":22992,"parent_record_offset":22968,"ancestor_record_offset":19324,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":20180,"parent_record_offset":20072,"ancestor_record_offset":19324,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":555,"record_offset":23108,"parent_record_offset":23076,"ancestor_record_offset":19324,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":19324,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":20424,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":19324,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":19324,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":19324,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":19324,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":19744,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":555,"record_offset":20072,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":555,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":19324,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":555,"record_offset":22968,"parent_record_offset":22944,"ancestor_record_offset":19324,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":555,"record_offset":20304,"parent_record_offset":19692,"ancestor_record_offset":19324,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":555,"record_offset":22392,"parent_record_offset":22368,"ancestor_record_offset":19324,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$160::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
//     evidence={"module":555,"record_offset":24308,"parent_record_offset":24284,"ancestor_record_offset":19324,"inlinee":34684,"source_line":760,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":27332,"parent_record_offset":27136,"ancestor_record_offset":19324,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":27172,"parent_record_offset":27136,"ancestor_record_offset":19324,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":24184,"parent_record_offset":24156,"ancestor_record_offset":19324,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":24132,"parent_record_offset":24104,"ancestor_record_offset":19324,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":555,"record_offset":27200,"parent_record_offset":27172,"ancestor_record_offset":19324,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":555,"record_offset":24212,"parent_record_offset":24184,"ancestor_record_offset":19324,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":555,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":19324,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":555,"record_offset":26168,"parent_record_offset":26144,"ancestor_record_offset":19324,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00555.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_lists.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 62
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "ref$", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "core::option::Option", "arguments": ["ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "enum2$", "arguments": ["core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "constructor": "tuple$", "arguments": ["enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >", "ref$<enum2$<country_core::components::drawable::DrawableMeshData> >", "ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>", "ref$<country_core::components::drawable::MaterialArgs>", "enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >", "enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >", "constructor": "bevy_ecs::query::state::QueryState", "arguments": ["tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "bevy_ecs::query::filter::With<utils::components::transform::Transform>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::query::state::QueryState<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>", "constructor": "tuple$", "arguments": ["country_core::render::nani::GpuDrivenDrawable", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::drawable::OcclusionCulled", "country_core::render::nani::NaniSubsetDrawable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ptr::MovingPtr<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>,bevy_ptr::Aligned>", "constructor": "bevy_ptr::MovingPtr", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>", "bevy_ptr::Aligned"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ptr::MovingPtr<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>,bevy_ptr::Aligned> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ptr::MovingPtr<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>,bevy_ptr::Aligned>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$19::component_ids<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >", "constructor": "bevy_ecs::bundle::impls::impl$19::component_ids", "arguments": ["country_core::render::nani::GpuDrivenDrawable", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::drawable::OcclusionCulled", "country_core::render::nani::NaniSubsetDrawable", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::clone::component_clone_via_clone<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::component::clone::component_clone_via_clone", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::iter::QueryIterationCursor::init<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >", "constructor": "bevy_ecs::query::iter::QueryIterationCursor::init", "arguments": ["tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "bevy_ecs::query::filter::With<utils::components::transform::Transform>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::drawable::OcclusionCulled"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::KeepOnLevelUnload", "country_core::components::drawable::DontCastShadow"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::drawable::DontCastShadow", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "tuple$", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_archetype<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >", "constructor": "bevy_ecs::query::state::QueryState::new_archetype", "arguments": ["tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "bevy_ecs::query::filter::With<utils::components::transform::Transform>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >", "constructor": "bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell", "arguments": ["tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "bevy_ecs::query::filter::With<utils::components::transform::Transform>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new<tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >,bevy_ecs::query::filter::With<utils::components::transform::Transform> >", "constructor": "bevy_ecs::query::state::QueryState::new", "arguments": ["tuple$<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "bevy_ecs::query::filter::With<utils::components::transform::Transform>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_component<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::world::World::register_component", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::register::ComponentsRegistrator::register_component_checked<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>", "constructor": "bevy_ecs::component::register::ComponentsRegistrator::register_component_checked", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::world_query::impl$12::init_state<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "constructor": "bevy_ecs::query::world_query::impl$12::init_state", "arguments": ["enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >", "ref$<enum2$<country_core::components::drawable::DrawableMeshData> >", "ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>", "ref$<country_core::components::drawable::MaterialArgs>", "enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >", "enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::world_query::impl$12::update_component_access<enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >,ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<country_core::components::drawable::MaterialArgs>,enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >,enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > > >", "constructor": "bevy_ecs::query::world_query::impl$12::update_component_access", "arguments": ["enum2$<core::option::Option<ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > > >", "ref$<enum2$<country_core::components::drawable::DrawableMeshData> >", "ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>", "ref$<country_core::components::drawable::MaterialArgs>", "enum2$<core::option::Option<ref$<country_core::components::drawable::DrawLineWidth> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > >", "enum2$<core::option::Option<ref$<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "enum2$<core::option::Option<ref$<country_core::components::drawable::Instanced> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$11::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::bundle::impls::impl$11::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$15::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >", "constructor": "bevy_ecs::bundle::impls::impl$15::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::drawable::OcclusionCulled", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$15::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >", "constructor": "bevy_ecs::bundle::impls::impl$15::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::drawable::DontCastShadow", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$19::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impls::impl$19::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn", "country_core::components::KeepOnLevelUnload", "country_core::components::drawable::DontCastShadow", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::render::nani::GpuDrivenDrawable,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled,country_core::render::nani::NaniSubsetDrawable> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::drawable::OcclusionCulled> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::CustomDrawFn>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_lists::DrawList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_lists::DrawList>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by_key::closure_env$0", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "u32", "country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>,alloc::vec::Vec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>", "alloc::vec::Vec<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem", "alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::systems::render_loop::draw_passes::draw_lists::DrawListItem,u32,country_core::systems::render_loop::draw_passes::draw_lists::impl$1::sort::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_lists.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: draw_lists.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5692800,"caller_aliases":{"alias_group_rva":5692800,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5694982,"position_among_mapped_render_targets":1,"mapped_render_target_count":4,"previous_callsite":null,"next_callsite":5694998,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5692800,"caller_aliases":{"alias_group_rva":5692800,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5694998,"position_among_mapped_render_targets":2,"mapped_render_target_count":4,"previous_callsite":5694982,"next_callsite":5695014,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5692800,"caller_aliases":{"alias_group_rva":5692800,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5695014,"position_among_mapped_render_targets":3,"mapped_render_target_count":4,"previous_callsite":5694998,"next_callsite":5699669,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/image_baker.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9869216,"caller_aliases":{"alias_group_rva":9869216,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9869485,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_jump_outside_procedure"}
// [父函数静态调用位置] {"caller_entry_rva":11417424,"caller_aliases":{"alias_group_rva":11417424,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":11423720,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24460224,"caller_aliases":{"alias_group_rva":24460224,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24460679,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24466608,"caller_aliases":{"alias_group_rva":24466608,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24467064,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24468016,"caller_aliases":{"alias_group_rva":24468016,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24468478,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24482144,"caller_aliases":{"alias_group_rva":24482144,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24482606,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24483552,"caller_aliases":{"alias_group_rva":24483552,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24484014,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24484960,"caller_aliases":{"alias_group_rva":24484960,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24485422,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24501808,"caller_aliases":{"alias_group_rva":24501808,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24502264,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24506512,"caller_aliases":{"alias_group_rva":24506512,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24506986,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24511872,"caller_aliases":{"alias_group_rva":24511872,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24512334,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24514752,"caller_aliases":{"alias_group_rva":24514752,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24515214,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24516160,"caller_aliases":{"alias_group_rva":24516160,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24516622,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24519024,"caller_aliases":{"alias_group_rva":24519024,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24519480,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24528912,"caller_aliases":{"alias_group_rva":24528912,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24529367,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24531760,"caller_aliases":{"alias_group_rva":24531760,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24532222,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24536800,"caller_aliases":{"alias_group_rva":24536800,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24537262,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24538208,"caller_aliases":{"alias_group_rva":24538208,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24538663,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":24548848,"caller_aliases":{"alias_group_rva":24548848,"count":1,"display_examples":["rhapsody::rg::api::GraphApi::raster"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":24549304,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_lists.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
// 信息 ID: 582e935877139c448686ac3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56e606 -> 0xae5310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
// 信息 ID: 36912c795284200a529be613
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56e626 -> 0xae5490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
// 信息 ID: 9b8891b7ff26083e9fdc633f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56e616 -> 0xae3750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$0
// 信息 ID: dccf01b49da886fb84bb00b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists; module=555, inline_record=50136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$1
// 信息 ID: b51c57d80a5514da08450805
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists @ 0xae4fe8 -> 0x2d78790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn
// 信息 ID: 33e45b8a6bf7eb26b9abd4ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks @ 0x9698ad -> 0xae2cb0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn::closure$0
// 信息 ID: 332c6a3a4e4df9d3f12947af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn; module=555, inline_record=18836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roof_tile_ssbo::RoofSpireSsbo>
// 信息 ID: 0c8898b16a8141c70af314c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn -> ["country_core::resources::roof_tile_ssbo::RoofSpireSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roofs::roof_buffers::RoofSupportBeamSsbo>
// 信息 ID: 971716d361ff427bd38c05d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn -> ["country_core::resources::roofs::roof_buffers::RoofSupportBeamSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>
// 信息 ID: b3de66fe76c4c76199fd6b80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn -> ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>
// 信息 ID: 35798d4ecf4a5c88b1951388
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn -> ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$0::multi_instanced_indexed::draw_fn<country_core::resources::sparkle::SparkleSsbo>
// 信息 ID: e2e2eeac2f8306c8a6c34abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::multi_instanced_indexed::draw_fn -> ["country_core::resources::sparkle::SparkleSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$15::default
// 信息 ID: 828d8c6663f9fc0af4d878b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new; module=555, inline_record=56240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::impl$4::clone
// 信息 ID: 91541c153d72fd42c5cf5bc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists; module=555, inline_record=53460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_lists::render_bucket
// 信息 ID: 9caebe7c79a287ce046c020d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3> @ 0x1753d87 -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3> @ 0x1755678 -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1> @ 0x1755bfe -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1> @ 0x175932e -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0> @ 0x17598ae -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4> @ 0x1759e2e -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0> @ 0x175dff8 -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment>,country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass::closure$0::closure$0::closure_env$0> @ 0x175f26a -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2> @ 0x176074e -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5> @ 0x176128e -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2> @ 0x176180e -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2> @ 0x1762338 -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass::closure_env$0> @ 0x17649d7 -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0> @ 0x17654fe -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4> @ 0x17668ae -> 0xae3020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_pass::ui_pass::closure_env$0> @ 0x1766e27 -> 0xae3020 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
