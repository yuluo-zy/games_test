// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/wall_state/spatial_hash_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 402a4e3f71d145d93d36f244c33e696d22ab7597b5bb7a66dc9f9e7c5b617a75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spatial_hash_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 497
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_state::spatial_hash_update::spatial_hash_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":43232,"rva":7127824,"va":5375836944}
// 编译实例 RVAs: 0x6cc310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: spatial_hash_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6224607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e82105a1ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375838122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7470678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e856fe7100","size":5,"soft_fail":false,"target_va":5383308144,"va":5375837461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141084b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spatial_hash_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10191347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8f3819b00","size":5,"soft_fail":false,"target_va":5386029904,"va":5375838552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27138611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8331a9e01","size":5,"soft_fail":false,"target_va":5402978688,"va":5375840072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27138857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8291b9e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375840018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27139169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8611c9e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375839978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t27139570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8f21d9e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5375839769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e810acffff","size":5,"soft_fail":false,"target_va":5375818176,"va":5375839659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27142613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8d5299e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375837510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-18731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8d5b6ffff","size":5,"soft_fail":false,"target_va":5375818368,"va":5375837094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27143480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8382d9e01","size":5,"soft_fail":false,"target_va":5402980656,"va":5375837171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::add_segment
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1022902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e84a64f0ff","size":5,"soft_fail":false,"target_va":5374816496,"va":5375839393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::remove_wall_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1024174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8525ff0ff","size":5,"soft_fail":false,"target_va":5374814208,"va":5375838377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1515774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e8fe201700","size":5,"soft_fail":false,"target_va":5377353776,"va":5375837997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t519233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e841ec0700","size":5,"soft_fail":false,"target_va":5376356912,"va":5375837674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t523114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e86afb0700","size":5,"soft_fail":false,"target_va":5376360560,"va":5375837441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t519065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375836944,"raw_hex":"e899eb0700","size":5,"soft_fail":false,"target_va":5376356144,"va":5375837074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":48268,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":48088,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":497,"record_offset":46108,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":497,"record_offset":46712,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":51992,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":497,"record_offset":46656,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":497,"record_offset":48172,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":52936,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":497,"record_offset":47024,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":497,"record_offset":46684,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":497,"record_offset":51908,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":45060,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":43472,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":44048,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_approx_coord_from_pos
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":48688,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":21778,"source_line":566,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":51196,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":48144,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":48364,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":27900,"source_line":133,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":49716,"parent_record_offset":43232,"ancestor_record_offset":43232,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":51224,"parent_record_offset":51196,"ancestor_record_offset":43232,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":48112,"parent_record_offset":48088,"ancestor_record_offset":43232,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":497,"record_offset":46196,"parent_record_offset":46172,"ancestor_record_offset":43232,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":497,"record_offset":46172,"parent_record_offset":46132,"ancestor_record_offset":43232,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":497,"record_offset":46736,"parent_record_offset":46712,"ancestor_record_offset":43232,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":497,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":43232,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45460,"parent_record_offset":45432,"ancestor_record_offset":43232,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45512,"parent_record_offset":45488,"ancestor_record_offset":43232,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45488,"parent_record_offset":45460,"ancestor_record_offset":43232,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45400,"parent_record_offset":45372,"ancestor_record_offset":43232,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45432,"parent_record_offset":45400,"ancestor_record_offset":43232,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45604,"parent_record_offset":45432,"ancestor_record_offset":43232,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":497,"record_offset":45716,"parent_record_offset":45692,"ancestor_record_offset":43232,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":49644,"parent_record_offset":49620,"ancestor_record_offset":43232,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":49668,"parent_record_offset":49644,"ancestor_record_offset":43232,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":48228,"parent_record_offset":48200,"ancestor_record_offset":43232,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":47840,"parent_record_offset":47816,"ancestor_record_offset":43232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":47308,"parent_record_offset":47284,"ancestor_record_offset":43232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":47392,"parent_record_offset":47368,"ancestor_record_offset":43232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":49268,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":49240,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":47332,"parent_record_offset":47308,"ancestor_record_offset":43232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":497,"record_offset":46788,"parent_record_offset":46764,"ancestor_record_offset":43232,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":497,"record_offset":46764,"parent_record_offset":46736,"ancestor_record_offset":43232,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":497,"record_offset":46868,"parent_record_offset":46844,"ancestor_record_offset":43232,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":497,"record_offset":48944,"parent_record_offset":48916,"ancestor_record_offset":43232,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":497,"record_offset":48852,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":497,"record_offset":48916,"parent_record_offset":48884,"ancestor_record_offset":43232,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":497,"record_offset":48200,"parent_record_offset":48172,"ancestor_record_offset":43232,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":497,"record_offset":45536,"parent_record_offset":45512,"ancestor_record_offset":43232,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":46244,"parent_record_offset":46196,"ancestor_record_offset":43232,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":497,"record_offset":43684,"parent_record_offset":43588,"ancestor_record_offset":43232,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":497,"record_offset":47504,"parent_record_offset":47480,"ancestor_record_offset":43232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":49352,"parent_record_offset":49324,"ancestor_record_offset":43232,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":49380,"parent_record_offset":49324,"ancestor_record_offset":43232,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":49096,"parent_record_offset":49072,"ancestor_record_offset":43232,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":497,"record_offset":49620,"parent_record_offset":49596,"ancestor_record_offset":43232,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":497,"record_offset":49596,"parent_record_offset":49568,"ancestor_record_offset":43232,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":46812,"parent_record_offset":46788,"ancestor_record_offset":43232,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":45296,"parent_record_offset":45268,"ancestor_record_offset":43232,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":48444,"parent_record_offset":48364,"ancestor_record_offset":43232,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":44644,"parent_record_offset":44184,"ancestor_record_offset":43232,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":45344,"parent_record_offset":45168,"ancestor_record_offset":43232,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":46844,"parent_record_offset":46764,"ancestor_record_offset":43232,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":43996,"parent_record_offset":43968,"ancestor_record_offset":43232,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":45668,"parent_record_offset":45372,"ancestor_record_offset":43232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":46548,"parent_record_offset":46524,"ancestor_record_offset":43232,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":47752,"parent_record_offset":47728,"ancestor_record_offset":43232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":47644,"parent_record_offset":47620,"ancestor_record_offset":43232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":46968,"parent_record_offset":46944,"ancestor_record_offset":43232,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":43232,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":44396,"parent_record_offset":44372,"ancestor_record_offset":43232,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":44604,"parent_record_offset":44572,"ancestor_record_offset":43232,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":45560,"parent_record_offset":45536,"ancestor_record_offset":43232,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":46068,"parent_record_offset":45132,"ancestor_record_offset":43232,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":43620,"parent_record_offset":43588,"ancestor_record_offset":43232,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":46500,"parent_record_offset":46196,"ancestor_record_offset":43232,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":48296,"parent_record_offset":48268,"ancestor_record_offset":43232,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":43648,"parent_record_offset":43620,"ancestor_record_offset":43232,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":48324,"parent_record_offset":48296,"ancestor_record_offset":43232,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":46372,"parent_record_offset":46348,"ancestor_record_offset":43232,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":46348,"parent_record_offset":46308,"ancestor_record_offset":43232,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":497,"record_offset":46524,"parent_record_offset":46500,"ancestor_record_offset":43232,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":46944,"parent_record_offset":46920,"ancestor_record_offset":43232,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":44372,"parent_record_offset":44348,"ancestor_record_offset":43232,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":44700,"parent_record_offset":44676,"ancestor_record_offset":43232,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":46920,"parent_record_offset":46868,"ancestor_record_offset":43232,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":44348,"parent_record_offset":44240,"ancestor_record_offset":43232,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":497,"record_offset":43588,"parent_record_offset":43560,"ancestor_record_offset":43232,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":497,"record_offset":44080,"parent_record_offset":44048,"ancestor_record_offset":43232,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":497,"record_offset":44212,"parent_record_offset":44184,"ancestor_record_offset":43232,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":497,"record_offset":44184,"parent_record_offset":44152,"ancestor_record_offset":43232,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":497,"record_offset":44128,"parent_record_offset":44104,"ancestor_record_offset":43232,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":497,"record_offset":44152,"parent_record_offset":44128,"ancestor_record_offset":43232,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":497,"record_offset":44104,"parent_record_offset":44080,"ancestor_record_offset":43232,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":497,"record_offset":44676,"parent_record_offset":44152,"ancestor_record_offset":43232,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43784,"parent_record_offset":43752,"ancestor_record_offset":43232,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43944,"parent_record_offset":43784,"ancestor_record_offset":43232,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43752,"parent_record_offset":43728,"ancestor_record_offset":43232,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43504,"parent_record_offset":43472,"ancestor_record_offset":43232,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43968,"parent_record_offset":43944,"ancestor_record_offset":43232,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43560,"parent_record_offset":43528,"ancestor_record_offset":43232,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":43528,"parent_record_offset":43504,"ancestor_record_offset":43232,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":497,"record_offset":44240,"parent_record_offset":44212,"ancestor_record_offset":43232,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":497,"record_offset":44488,"parent_record_offset":44240,"ancestor_record_offset":43232,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":497,"record_offset":44460,"parent_record_offset":44240,"ancestor_record_offset":43232,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":497,"record_offset":44292,"parent_record_offset":44240,"ancestor_record_offset":43232,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":51932,"parent_record_offset":51908,"ancestor_record_offset":43232,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":51956,"parent_record_offset":51932,"ancestor_record_offset":43232,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":497,"record_offset":47864,"parent_record_offset":47840,"ancestor_record_offset":43232,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":49416,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":49184,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":49300,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":49072,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":49324,"parent_record_offset":49300,"ancestor_record_offset":43232,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":51668,"parent_record_offset":51644,"ancestor_record_offset":43232,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":51692,"parent_record_offset":51668,"ancestor_record_offset":43232,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":497,"record_offset":47816,"parent_record_offset":47792,"ancestor_record_offset":43232,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":47084,"parent_record_offset":47060,"ancestor_record_offset":43232,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":47108,"parent_record_offset":47084,"ancestor_record_offset":43232,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":47792,"parent_record_offset":47572,"ancestor_record_offset":43232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":47136,"parent_record_offset":47108,"ancestor_record_offset":43232,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47704,"parent_record_offset":47680,"ancestor_record_offset":43232,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47680,"parent_record_offset":47572,"ancestor_record_offset":43232,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47620,"parent_record_offset":47596,"ancestor_record_offset":43232,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":48000,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47596,"parent_record_offset":47572,"ancestor_record_offset":43232,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47188,"parent_record_offset":47164,"ancestor_record_offset":43232,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47164,"parent_record_offset":47108,"ancestor_record_offset":43232,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47212,"parent_record_offset":47188,"ancestor_record_offset":43232,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":497,"record_offset":47452,"parent_record_offset":47424,"ancestor_record_offset":43232,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":497,"record_offset":47480,"parent_record_offset":47452,"ancestor_record_offset":43232,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":497,"record_offset":47540,"parent_record_offset":47424,"ancestor_record_offset":43232,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":497,"record_offset":47424,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47256,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":47572,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":497,"record_offset":47284,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":497,"record_offset":47368,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":497,"record_offset":47908,"parent_record_offset":47212,"ancestor_record_offset":43232,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":497,"record_offset":48824,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":497,"record_offset":48884,"parent_record_offset":48852,"ancestor_record_offset":43232,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":45096,"parent_record_offset":45060,"ancestor_record_offset":43232,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":497,"record_offset":45208,"parent_record_offset":45168,"ancestor_record_offset":43232,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":45372,"parent_record_offset":45168,"ancestor_record_offset":43232,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":497,"record_offset":45640,"parent_record_offset":45372,"ancestor_record_offset":43232,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":47060,"parent_record_offset":47024,"ancestor_record_offset":43232,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":497,"record_offset":49528,"parent_record_offset":49496,"ancestor_record_offset":43232,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":497,"record_offset":45236,"parent_record_offset":45208,"ancestor_record_offset":43232,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":497,"record_offset":45268,"parent_record_offset":45236,"ancestor_record_offset":43232,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":497,"record_offset":45168,"parent_record_offset":45132,"ancestor_record_offset":43232,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":497,"record_offset":45132,"parent_record_offset":45096,"ancestor_record_offset":43232,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":48056,"parent_record_offset":47024,"ancestor_record_offset":43232,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":49756,"parent_record_offset":49716,"ancestor_record_offset":43232,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":49848,"parent_record_offset":49716,"ancestor_record_offset":43232,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":49568,"parent_record_offset":48688,"ancestor_record_offset":43232,"inlinee":21779,"source_line":1056,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":497,"record_offset":51644,"parent_record_offset":51420,"ancestor_record_offset":43232,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00497.jsonl.gz"}
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
// 完整结构及原始类型记录: spatial_hash_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spatial_hash_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spatial_hash_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::spatial_hash_update::spatial_hash_update
// 信息 ID: 2b642242523e65d2a1a640de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9809299e3e6222c17eaa40c9: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>)> @ 0x50583 -> 0x6cc310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>)> > @ 0xa9d11 -> 0x6cc310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
