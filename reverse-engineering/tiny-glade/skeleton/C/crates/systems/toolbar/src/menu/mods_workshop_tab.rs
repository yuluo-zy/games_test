// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/mods_workshop_tab.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b1abc0524d2e03b10f13d71af377cee336b52b3ecf614b43d3e8598b1791d249
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mods_workshop_tab.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1267, 2873, 2897, 2912, 4209
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466","version":"0.11.0"}]
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::subscription_status
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2897,"record_offset":5564,"rva":37013168,"va":5405722288}
// 编译实例 RVAs: 0x234c6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-30044022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e88a9035fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5405722705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-30044961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8df8c35fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405723660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37000499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8cd6acbfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5405722686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32243359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8610114fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405722394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11334320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8b0f2ac00","size":5,"soft_fail":false,"target_va":5417057056,"va":5405722731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20928506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e806a8c0fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5405723365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10897739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e84b49a600","size":5,"soft_fail":false,"target_va":5416620794,"va":5405723050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<ref$<str$> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-3407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8b1f2ffff","size":5,"soft_fail":false,"target_va":5405720016,"va":5405723418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::status
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24479200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8207a8afe","size":5,"soft_fail":false,"target_va":5381243152,"va":5405722347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_clutter::steam_download_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24480802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8de738afe","size":5,"soft_fail":false,"target_va":5381241760,"va":5405722557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22587945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8d755a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405722660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::mods_workshop_tab::parts_label
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405722288,"raw_hex":"e8ccfcffff","size":5,"soft_fail":false,"target_va":5405721936,"va":5405722751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2897,"record_offset":6780,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":5728,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":6724,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":6752,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":10888,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":5780,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::mods_workshop_tab::format_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":2897,"record_offset":7732,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":111827,"source_line":94,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":5564,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":5564,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":11064,"parent_record_offset":11040,"ancestor_record_offset":5564,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":5564,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":5564,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":5564,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":7504,"parent_record_offset":7460,"ancestor_record_offset":5564,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":11040,"parent_record_offset":10984,"ancestor_record_offset":5564,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":7772,"parent_record_offset":7732,"ancestor_record_offset":5564,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":5564,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":5564,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":11012,"parent_record_offset":10984,"ancestor_record_offset":5564,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":5564,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":5564,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":5564,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":5564,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":5564,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2897,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":5564,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2897,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":5564,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2897,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":5564,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2897,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":5564,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5564,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2897,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":5564,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2897,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":5564,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":5564,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":5564,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2897,"record_offset":6552,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2897,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":5564,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2897,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":5564,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":5564,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":5564,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":5564,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2897,"record_offset":7648,"parent_record_offset":7304,"ancestor_record_offset":5564,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5564,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":10912,"parent_record_offset":10888,"ancestor_record_offset":5564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2897,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":5564,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":2897,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":5564,"inlinee":35007,"source_line":25,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":5564,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":6228,"parent_record_offset":6196,"ancestor_record_offset":5564,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":6168,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":6028,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":6580,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":6112,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2897,"record_offset":5972,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2897,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":5564,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":5564,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":6436,"parent_record_offset":6328,"ancestor_record_offset":5564,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":5840,"parent_record_offset":5808,"ancestor_record_offset":5564,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":5564,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":6672,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":5564,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":5564,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":5868,"parent_record_offset":5808,"ancestor_record_offset":5564,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":5564,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":6000,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":6328,"parent_record_offset":5920,"ancestor_record_offset":5564,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_workshop_tab::subscription_status::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":2897,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":5564,"inlinee":111825,"source_line":210,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::parts_label
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2897,"record_offset":4316,"rva":37012816,"va":5405721936}
// 编译实例 RVAs: 0x234c550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30043502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721936,"raw_hex":"e8929235fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405722201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10648823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721936,"raw_hex":"e8f77ca200","size":5,"soft_fail":false,"target_va":5416370864,"va":5405722036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$alloc..slice..Join$LT$$RF$str$GT$$u20$for$u20$$u5b$S$u5d$$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6611087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721936,"raw_hex":"e88fe06400","size":5,"soft_fail":false,"target_va":5412333264,"va":5405722172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22587295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721936,"raw_hex":"e86158a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405722010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2897,"record_offset":4948,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":11201,"source_line":596,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":4464,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":4520,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":4492,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":4976,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":4316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":4316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":5140,"parent_record_offset":5048,"ancestor_record_offset":4316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":4608,"parent_record_offset":4576,"ancestor_record_offset":4316,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4316,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":4316,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":4316,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":4576,"parent_record_offset":4544,"ancestor_record_offset":4316,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":4316,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2897,"record_offset":5108,"parent_record_offset":5076,"ancestor_record_offset":4316,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":4696,"parent_record_offset":4544,"ancestor_record_offset":4316,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::format_size
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4D3
// 首条位置: {"module":2897,"inlinee":111827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":7772,"parent_record_offset":7732,"ancestor_record_offset":7732,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":7732,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":7732,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2897,"record_offset":17524,"rva":37016176,"va":5405725296}
// 编译实例 RVAs: 0x234d270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-30047391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8618335fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405726090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36065127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e899b0d9fd","size":5,"soft_fail":false,"target_va":5369660368,"va":5405725490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32247439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e871f113fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405726474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32241829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e85b0714fe","size":5,"soft_fail":false,"target_va":5373484384,"va":5405726208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32241458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8ce0814fe","size":5,"soft_fail":false,"target_va":5373484784,"va":5405726237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29896584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e878d037fe","size":5,"soft_fail":false,"target_va":5375829056,"va":5405725635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26925594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8e62565fe","size":5,"soft_fail":false,"target_va":5378801568,"va":5405727157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24662003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e80db087fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405726174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18702792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8389ee2fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5405726339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18700828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8e4a5e2fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5405726279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18681822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e822f0e2fe","size":5,"soft_fail":false,"target_va":5387044016,"va":5405725833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141181740
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18661937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8cf3de3fe","size":5,"soft_fail":false,"target_va":5387065152,"va":5405727084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18521680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8b061e5fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5405726363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18520594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8ee65e5fe","size":5,"soft_fail":false,"target_va":5387205712,"va":5405726301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c1580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18398584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e88842e7fe","size":5,"soft_fail":false,"target_va":5387326848,"va":5405725427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14231c060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-201835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e895ebfcff","size":5,"soft_fail":false,"target_va":5405524064,"va":5405725894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142369dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t117302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e836ca0100","size":5,"soft_fail":false,"target_va":5405842880,"va":5405725573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11329279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8ffdeac00","size":5,"soft_fail":false,"target_va":5417054928,"va":5405725644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11329490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8d2dfac00","size":5,"soft_fail":false,"target_va":5417055168,"va":5405725673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20930856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8d89ec0fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5405725715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23728332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e834ef95fe","size":5,"soft_fail":false,"target_va":5381997408,"va":5405725735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10894657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8413da600","size":5,"soft_fail":false,"target_va":5416621600,"va":5405726938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22591428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e83c48a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405726143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10892395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e86b34a600","size":5,"soft_fail":false,"target_va":5416619360,"va":5405726960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22821554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e84ec5a3fe","size":5,"soft_fail":false,"target_va":5382903888,"va":5405725437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29517897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e8b7973dfe","size":5,"soft_fail":false,"target_va":5376208368,"va":5405726260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::mods_workshop_tab::browse_workshop_button
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405725296,"raw_hex":"e83ef9ffff","size":5,"soft_fail":false,"target_va":5405724192,"va":5405725917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":19524,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":17956,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2897,"record_offset":19556,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2897,"record_offset":17872,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2897,"record_offset":18072,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":18688,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":18716,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":19584,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2897,"record_offset":17704,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":22300,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":21164,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":21104,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":20900,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":21224,"parent_record_offset":17524,"ancestor_record_offset":17524,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":17524,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":17524,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":20784,"parent_record_offset":20760,"ancestor_record_offset":17524,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":20712,"parent_record_offset":20656,"ancestor_record_offset":17524,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":18972,"parent_record_offset":18756,"ancestor_record_offset":17524,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":17524,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":20684,"parent_record_offset":20656,"ancestor_record_offset":17524,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":17524,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":20632,"parent_record_offset":20608,"ancestor_record_offset":17524,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":18844,"parent_record_offset":18816,"ancestor_record_offset":17524,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":18816,"parent_record_offset":18788,"ancestor_record_offset":17524,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":17524,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2897,"record_offset":17896,"parent_record_offset":17872,"ancestor_record_offset":17524,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2897,"record_offset":18380,"parent_record_offset":18352,"ancestor_record_offset":17524,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2897,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":17524,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2897,"record_offset":18408,"parent_record_offset":18352,"ancestor_record_offset":17524,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2897,"record_offset":22024,"parent_record_offset":22000,"ancestor_record_offset":17524,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":19876,"parent_record_offset":19852,"ancestor_record_offset":17524,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":21592,"parent_record_offset":21568,"ancestor_record_offset":17524,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":17524,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2897,"record_offset":19272,"parent_record_offset":18756,"ancestor_record_offset":17524,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2897,"record_offset":22092,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2897,"record_offset":18096,"parent_record_offset":18072,"ancestor_record_offset":17524,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2897,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":17524,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2897,"record_offset":17920,"parent_record_offset":17896,"ancestor_record_offset":17524,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2897,"record_offset":18588,"parent_record_offset":18564,"ancestor_record_offset":17524,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":2897,"record_offset":17984,"parent_record_offset":17956,"ancestor_record_offset":17524,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":2897,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":17524,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2897,"record_offset":18456,"parent_record_offset":18432,"ancestor_record_offset":17524,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2897,"record_offset":23884,"parent_record_offset":23856,"ancestor_record_offset":17524,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2897,"record_offset":23912,"parent_record_offset":23884,"ancestor_record_offset":17524,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2897,"record_offset":24264,"parent_record_offset":24232,"ancestor_record_offset":17524,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":17524,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":17524,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":17524,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":22264,"parent_record_offset":21256,"ancestor_record_offset":17524,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2897,"record_offset":19908,"parent_record_offset":19824,"ancestor_record_offset":17524,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2897,"record_offset":18324,"parent_record_offset":18296,"ancestor_record_offset":17524,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":17524,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":18652,"parent_record_offset":18164,"ancestor_record_offset":17524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2897,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":17524,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2897,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":17524,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2897,"record_offset":20244,"parent_record_offset":20220,"ancestor_record_offset":17524,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2897,"record_offset":21292,"parent_record_offset":21256,"ancestor_record_offset":17524,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2897,"record_offset":18876,"parent_record_offset":18844,"ancestor_record_offset":17524,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2897,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":17524,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":18904,"parent_record_offset":18844,"ancestor_record_offset":17524,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":19200,"parent_record_offset":19140,"ancestor_record_offset":17524,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":19232,"parent_record_offset":19104,"ancestor_record_offset":17524,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2897,"record_offset":22048,"parent_record_offset":22024,"ancestor_record_offset":17524,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":20040,"parent_record_offset":20012,"ancestor_record_offset":17524,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":20072,"parent_record_offset":20040,"ancestor_record_offset":17524,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":20012,"parent_record_offset":19972,"ancestor_record_offset":17524,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":21568,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":19852,"parent_record_offset":19824,"ancestor_record_offset":17524,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":22120,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":21652,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2897,"record_offset":21512,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2897,"record_offset":22000,"parent_record_offset":21976,"ancestor_record_offset":17524,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":21348,"parent_record_offset":21320,"ancestor_record_offset":17524,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":21976,"parent_record_offset":21868,"ancestor_record_offset":17524,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":17524,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":21380,"parent_record_offset":21348,"ancestor_record_offset":17524,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":20368,"parent_record_offset":19736,"ancestor_record_offset":17524,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":17524,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":20220,"parent_record_offset":19972,"ancestor_record_offset":17524,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":22212,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19824,"parent_record_offset":19768,"ancestor_record_offset":17524,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":17524,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":17524,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21432,"parent_record_offset":21408,"ancestor_record_offset":17524,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21408,"parent_record_offset":21348,"ancestor_record_offset":17524,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":23824,"parent_record_offset":23792,"ancestor_record_offset":17524,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19796,"parent_record_offset":19768,"ancestor_record_offset":17524,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19736,"parent_record_offset":19680,"ancestor_record_offset":17524,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19680,"parent_record_offset":19656,"ancestor_record_offset":17524,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21456,"parent_record_offset":21432,"ancestor_record_offset":17524,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":23792,"parent_record_offset":23768,"ancestor_record_offset":17524,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":17524,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19768,"parent_record_offset":19736,"ancestor_record_offset":17524,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21540,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":23856,"parent_record_offset":23824,"ancestor_record_offset":17524,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19656,"parent_record_offset":19632,"ancestor_record_offset":17524,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":19944,"parent_record_offset":19736,"ancestor_record_offset":17524,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":21868,"parent_record_offset":21456,"ancestor_record_offset":17524,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":21320,"parent_record_offset":21256,"ancestor_record_offset":17524,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":18512,"parent_record_offset":18164,"ancestor_record_offset":17524,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":18164,"parent_record_offset":18128,"ancestor_record_offset":17524,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":18540,"parent_record_offset":18164,"ancestor_record_offset":17524,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2897,"record_offset":18236,"parent_record_offset":18208,"ancestor_record_offset":17524,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2897,"record_offset":18352,"parent_record_offset":18296,"ancestor_record_offset":17524,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2897,"record_offset":18208,"parent_record_offset":18164,"ancestor_record_offset":17524,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2897,"record_offset":18296,"parent_record_offset":18264,"ancestor_record_offset":17524,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2897,"record_offset":18264,"parent_record_offset":18236,"ancestor_record_offset":17524,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::Client::utils
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":2897,"record_offset":18788,"parent_record_offset":18756,"ancestor_record_offset":17524,"inlinee":13662,"source_line":319,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::utils::Utils::app_id
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/utils.rs
//     evidence={"module":2897,"record_offset":18944,"parent_record_offset":18756,"ancestor_record_offset":17524,"inlinee":29524,"source_line":145,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":21012,"parent_record_offset":20952,"ancestor_record_offset":17524,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":20952,"parent_record_offset":20900,"ancestor_record_offset":17524,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":21256,"parent_record_offset":21224,"ancestor_record_offset":17524,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2897,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":17524,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":17524,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":20984,"parent_record_offset":20952,"ancestor_record_offset":17524,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":17524,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":21132,"parent_record_offset":21104,"ancestor_record_offset":17524,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":21068,"parent_record_offset":20952,"ancestor_record_offset":17524,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_workshop_tab::list_slot::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":2897,"record_offset":18756,"parent_record_offset":18716,"ancestor_record_offset":17524,"inlinee":111828,"source_line":137,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11419
// 首条位置: {"module":1267,"inlinee":70681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1267,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":27448,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1267,"record_offset":27556,"parent_record_offset":27500,"ancestor_record_offset":27448,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1267,"record_offset":27528,"parent_record_offset":27500,"ancestor_record_offset":27448,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1267,"record_offset":27500,"parent_record_offset":27472,"ancestor_record_offset":27448,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B500
// 首条位置: {"module":2912,"inlinee":111872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2912,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":1400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2912,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":1400,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2912,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1400,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2912,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1400,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2912,"record_offset":1608,"parent_record_offset":1552,"ancestor_record_offset":1400,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2912,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":1400,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2912,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":1400,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2912,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":1400,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2912,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4D4
// 首条位置: {"module":2897,"inlinee":111828}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":18972,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2897,"record_offset":19272,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":19056,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::utils
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":2897,"record_offset":18788,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":13662,"source_line":319,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] steamworks::utils::Utils::app_id
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/utils.rs
//     evidence={"module":2897,"record_offset":18944,"parent_record_offset":18756,"ancestor_record_offset":18756,"inlinee":29524,"source_line":145,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":18756,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":18844,"parent_record_offset":18816,"ancestor_record_offset":18756,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":18816,"parent_record_offset":18788,"ancestor_record_offset":18756,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2897,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":18756,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":18756,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":18756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2897,"record_offset":18876,"parent_record_offset":18844,"ancestor_record_offset":18756,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":18904,"parent_record_offset":18844,"ancestor_record_offset":18756,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":19200,"parent_record_offset":19140,"ancestor_record_offset":18756,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2897,"record_offset":19232,"parent_record_offset":19104,"ancestor_record_offset":18756,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B38D
// 首条位置: {"module":2873,"inlinee":111501}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":31148,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31424,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31456,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31276,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31244,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":31516,"parent_record_offset":31068,"ancestor_record_offset":31068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":31740,"parent_record_offset":31716,"ancestor_record_offset":31068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":31764,"parent_record_offset":31740,"ancestor_record_offset":31068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":31788,"parent_record_offset":31764,"ancestor_record_offset":31068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":31716,"parent_record_offset":31660,"ancestor_record_offset":31068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":31332,"parent_record_offset":31308,"ancestor_record_offset":31068,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":31380,"parent_record_offset":31356,"ancestor_record_offset":31068,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":31688,"parent_record_offset":31660,"ancestor_record_offset":31068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":31660,"parent_record_offset":31636,"ancestor_record_offset":31068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":31068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31356,"parent_record_offset":31332,"ancestor_record_offset":31068,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2873,"record_offset":31176,"parent_record_offset":31148,"ancestor_record_offset":31068,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2873,"record_offset":31204,"parent_record_offset":31176,"ancestor_record_offset":31068,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":2873,"record_offset":31308,"parent_record_offset":31276,"ancestor_record_offset":31068,"inlinee":111503,"source_line":163,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":2873,"record_offset":31484,"parent_record_offset":31456,"ancestor_record_offset":31068,"inlinee":111504,"source_line":170,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B38F
// 首条位置: {"module":2873,"inlinee":111503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":31332,"parent_record_offset":31308,"ancestor_record_offset":31308,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":31380,"parent_record_offset":31356,"ancestor_record_offset":31308,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":31356,"parent_record_offset":31332,"ancestor_record_offset":31308,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B390
// 首条位置: {"module":2873,"inlinee":111504}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x215F3
// 首条位置: {"module":4209,"inlinee":136691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":14436,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4209,"record_offset":17372,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":15764,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":15676,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14520,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14580,"parent_record_offset":14344,"ancestor_record_offset":14344,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":14344,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":15964,"parent_record_offset":15940,"ancestor_record_offset":14344,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":15988,"parent_record_offset":15964,"ancestor_record_offset":14344,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":15916,"parent_record_offset":15860,"ancestor_record_offset":14344,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":14344,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17600,"parent_record_offset":17568,"ancestor_record_offset":14344,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":15888,"parent_record_offset":15860,"ancestor_record_offset":14344,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":14344,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17648,"parent_record_offset":17624,"ancestor_record_offset":14344,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17624,"parent_record_offset":17600,"ancestor_record_offset":14344,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":14344,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17568,"parent_record_offset":17528,"ancestor_record_offset":14344,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17496,"parent_record_offset":17460,"ancestor_record_offset":14344,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17528,"parent_record_offset":17496,"ancestor_record_offset":14344,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4209,"record_offset":17432,"parent_record_offset":17396,"ancestor_record_offset":14344,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4209,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":14344,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4209,"record_offset":17460,"parent_record_offset":17396,"ancestor_record_offset":14344,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4209,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":14344,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":14992,"parent_record_offset":14968,"ancestor_record_offset":14344,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":15076,"parent_record_offset":15052,"ancestor_record_offset":14344,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":15492,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":15224,"parent_record_offset":15196,"ancestor_record_offset":14344,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":14344,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":14460,"parent_record_offset":14436,"ancestor_record_offset":14344,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":15016,"parent_record_offset":14992,"ancestor_record_offset":14344,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":15788,"parent_record_offset":15764,"ancestor_record_offset":14344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4209,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":14344,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":14644,"parent_record_offset":14612,"ancestor_record_offset":14344,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":17688,"parent_record_offset":17528,"ancestor_record_offset":14344,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4209,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":14344,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":15136,"parent_record_offset":15108,"ancestor_record_offset":14344,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":15168,"parent_record_offset":15136,"ancestor_record_offset":14344,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":15108,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":14968,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":15520,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":15052,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4209,"record_offset":14912,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4209,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":14344,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":14748,"parent_record_offset":14708,"ancestor_record_offset":14344,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":15376,"parent_record_offset":15268,"ancestor_record_offset":14344,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":14780,"parent_record_offset":14748,"ancestor_record_offset":14344,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":14344,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":15612,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":15292,"parent_record_offset":15268,"ancestor_record_offset":14344,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":14832,"parent_record_offset":14808,"ancestor_record_offset":14344,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":14808,"parent_record_offset":14748,"ancestor_record_offset":14344,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":14856,"parent_record_offset":14832,"ancestor_record_offset":14344,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":14940,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":15268,"parent_record_offset":14856,"ancestor_record_offset":14344,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":14708,"parent_record_offset":14672,"ancestor_record_offset":14344,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":14672,"parent_record_offset":14612,"ancestor_record_offset":14344,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":14612,"parent_record_offset":14580,"ancestor_record_offset":14344,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":15704,"parent_record_offset":15676,"ancestor_record_offset":14344,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":14344,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":14344,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::subscription_status::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4D1
// 首条位置: {"module":2897,"inlinee":111825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_workshop_tab::browse_workshop_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2897,"record_offset":13244,"rva":37015072,"va":5405724192}
// 编译实例 RVAs: 0x234ce20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-30046315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8958735fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405725014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32245376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e880f913fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405724411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32239951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8b10e14fe","size":5,"soft_fail":false,"target_va":5373484384,"va":5405724330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32239573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e82b1014fe","size":5,"soft_fail":false,"target_va":5373484784,"va":5405724352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29487893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8eb0c3efe","size":5,"soft_fail":false,"target_va":5376236864,"va":5405724752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24660128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e860b787fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405724299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18704031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e86199e2fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5405724762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18520096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8e067e5fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5405724779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422f5c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_workshop_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-357615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8118bfaff","size":5,"soft_fail":false,"target_va":5405367440,"va":5405725050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20929859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8bda2c0fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5405724718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10896432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e83044a600","size":5,"soft_fail":false,"target_va":5416621536,"va":5405725099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10896523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e88b44a600","size":5,"soft_fail":false,"target_va":5416621600,"va":5405725072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22589951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8014ea7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405724666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29516009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405724192,"raw_hex":"e8179f3dfe","size":5,"soft_fail":false,"target_va":5376208368,"va":5405724372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":14596,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2897,"record_offset":14680,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":15936,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":15852,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13412,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13552,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":14708,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":14768,"parent_record_offset":13244,"ancestor_record_offset":13244,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":13244,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":13244,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":16352,"parent_record_offset":16328,"ancestor_record_offset":13244,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2897,"record_offset":16280,"parent_record_offset":16224,"ancestor_record_offset":13244,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2897,"record_offset":14644,"parent_record_offset":14620,"ancestor_record_offset":13244,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":16076,"parent_record_offset":16048,"ancestor_record_offset":13244,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":16048,"parent_record_offset":16020,"ancestor_record_offset":13244,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2897,"record_offset":16020,"parent_record_offset":15992,"ancestor_record_offset":13244,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2897,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":13244,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":13892,"parent_record_offset":13868,"ancestor_record_offset":13244,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2897,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13244,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2897,"record_offset":14392,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":14124,"parent_record_offset":14096,"ancestor_record_offset":13244,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2897,"record_offset":14096,"parent_record_offset":14068,"ancestor_record_offset":13244,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":13244,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2897,"record_offset":14564,"parent_record_offset":13552,"ancestor_record_offset":13244,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":13244,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2897,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":13244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2897,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":13244,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2897,"record_offset":13592,"parent_record_offset":13552,"ancestor_record_offset":13244,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2897,"record_offset":14824,"parent_record_offset":14796,"ancestor_record_offset":13244,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2897,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":13244,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":14036,"parent_record_offset":14008,"ancestor_record_offset":13244,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":14068,"parent_record_offset":14036,"ancestor_record_offset":13244,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2897,"record_offset":14008,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":13868,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":14420,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2897,"record_offset":13952,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2897,"record_offset":13812,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2897,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":13244,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":14924,"parent_record_offset":14884,"ancestor_record_offset":13244,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":13648,"parent_record_offset":13620,"ancestor_record_offset":13244,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":14276,"parent_record_offset":14168,"ancestor_record_offset":13244,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2897,"record_offset":13680,"parent_record_offset":13648,"ancestor_record_offset":13244,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":13244,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":14512,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":13244,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":13244,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":14984,"parent_record_offset":14924,"ancestor_record_offset":13244,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":13708,"parent_record_offset":13648,"ancestor_record_offset":13244,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":13244,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":13840,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2897,"record_offset":14168,"parent_record_offset":13756,"ancestor_record_offset":13244,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":14884,"parent_record_offset":14852,"ancestor_record_offset":13244,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":13620,"parent_record_offset":13552,"ancestor_record_offset":13244,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2897,"record_offset":14852,"parent_record_offset":14796,"ancestor_record_offset":13244,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":14796,"parent_record_offset":14768,"ancestor_record_offset":13244,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2897,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":13244,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2897,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":13244,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13484,"parent_record_offset":13460,"ancestor_record_offset":13244,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":13244,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13516,"parent_record_offset":13436,"ancestor_record_offset":13244,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2897,"record_offset":13436,"parent_record_offset":13412,"ancestor_record_offset":13244,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02897.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2897,"record_offset":14736,"parent_record_offset":14708,"ancestor_record_offset":13244,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/02897.jsonl.gz"}
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
// 完整结构及原始类型记录: mods_workshop_tab.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mods_workshop_tab.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mods_workshop_tab.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::browse_workshop_button
// 信息 ID: a2ba2940e5cb36cf7226217e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::mods_workshop_tab::list_slot @ 0x234d4dd -> 0x234ce20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mods_workshop_tab::list_slot @ 0x234d7c0 -> 0x234ce20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::format_size
// 信息 ID: 606e5a75023e0916f6f063d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::mods_workshop_tab::subscription_status; module=2897, inline_record=7732; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::mods_workshop_tab::subscription_status; module=2897, inline_record=10684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot
// 信息 ID: 7465ac601cf78b8041a1fc89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::mods::ui::closure$4 @ 0x11a288b -> 0x234d270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$0
// 信息 ID: 0d1399387c4910faf4f21b85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::mods_workshop_tab::list_slot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription> >,toolbar::menu::mods_workshop_tab::list_slot::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription> >,toolbar::menu::mods_workshop_tab::list_slot::closure_env$0> >; module=1267, inline_record=27448; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription> >,toolbar::menu::mods_workshop_tab::list_slot::closure_env$0> >; module=1267, inline_record=27864; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription> >,toolbar::menu::mods_workshop_tab::list_slot::closure_env$0> >; module=1267, inline_record=28852; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription>,core::slice::iter::Iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription> >,toolbar::menu::mods_workshop_tab::list_slot::closure_env$0> >; module=1267, inline_record=29204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$1
// 信息 ID: 51ac65288f5f4aec71c074af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::mods_workshop_tab::list_slot::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::mods_workshop_tab::list_slot::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > > >; module=2912, inline_record=1400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$2
// 信息 ID: aac9b485d82395a79fe7a83d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::mods_workshop_tab::list_slot; module=2897, inline_record=18756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$3
// 信息 ID: 8d140022cd2adad4575b0510
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::mods_workshop_tab::list_slot::closure_env$3 <- toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3>; module=2873, inline_record=31068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$0
// 信息 ID: e390070894f91c8912816fd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3>; module=2873, inline_record=31308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$1
// 信息 ID: 4138dea5c3adddb6f715e56b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3>; module=2873, inline_record=31484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$2
// 信息 ID: 040cb3f7ef2da6cd634c2ce4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::mods_workshop_tab::list_slot", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2 <- toolbar::menu::mod_row::mod_row<toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::menu::mod_row::mod_row<toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2>; module=4209, inline_record=14344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::parts_label
// 信息 ID: 388d35c6de13dafebc3b3b8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_mine_tab::list_slot::closure_env$1> @ 0x231d354 -> 0x234c550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mods_workshop_tab::subscription_status @ 0x234c87f -> 0x234c550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::subscription_status
// 信息 ID: bb25722e7b1d72e66ce2316b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3> @ 0x231c7b2 -> 0x234c6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_workshop_tab::subscription_status::closure$0
// 信息 ID: f0db7d1da87cda782526d4c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::mods_workshop_tab::subscription_status", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::mods_workshop_tab::subscription_status; module=2897, inline_record=5752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
