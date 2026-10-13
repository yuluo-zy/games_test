// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/toolbar_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cf90f9eebe078cc5333c25196e580e76057009ff4b3741455d886f7479c19084
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3643, 3698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_update::toolbar_button_size_and_padding_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3643,"record_offset":3052,"rva":28677040,"va":5397386160}
// 编译实例 RVAs: 0x1b593b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9b620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t269465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8991c0400","size":5,"soft_fail":false,"target_va":5397657120,"va":5397387650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t274149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8e52e0400","size":5,"soft_fail":false,"target_va":5397661696,"va":5397387542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5590713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8b94e5500","size":5,"soft_fail":false,"target_va":5402978688,"va":5397387970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e82bf3ffff","size":5,"soft_fail":false,"target_va":5397384880,"va":5397388160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5593426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e852595500","size":5,"soft_fail":false,"target_va":5402980128,"va":5397386697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e850fbffff","size":5,"soft_fail":false,"target_va":5397385072,"va":5397386267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5594307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8c35c5500","size":5,"soft_fail":false,"target_va":5402980656,"va":5397386344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21029964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8b41bbffe","size":5,"soft_fail":false,"target_va":5376356912,"va":5397386871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21026073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8e72abffe","size":5,"soft_fail":false,"target_va":5376360560,"va":5397386628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21030108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8241bbffe","size":5,"soft_fail":false,"target_va":5376356144,"va":5397386247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h6a28a4a69af7475d (.llvm.7250520506814577795)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17764669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8c3eef0fe","size":5,"soft_fail":false,"target_va":5379621984,"va":5397386648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17627293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e86307f3fe","size":5,"soft_fail":false,"target_va":5379760416,"va":5397387704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17626794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e85609f3fe","size":5,"soft_fail":false,"target_va":5379760832,"va":5397387621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20449368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397386160,"raw_hex":"e8a8f7c7fe","size":5,"soft_fail":false,"target_va":5376938064,"va":5397387427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-2946","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5397386160,"raw_hex":"e97ef4ffff","size":5,"soft_fail":false,"target_va":5397384880,"va":5397387821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3643,"record_offset":7424,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3643,"record_offset":7340,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3643,"record_offset":7284,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3643,"record_offset":6444,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3643,"record_offset":6248,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3643,"record_offset":5880,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3643,"record_offset":7536,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3643,"record_offset":7172,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":4832,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":3244,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":3820,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3643,"record_offset":7396,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":84152,"source_line":66,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3643,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":3052,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3643,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":3052,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3643,"record_offset":6832,"parent_record_offset":6804,"ancestor_record_offset":3052,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3643,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":3052,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3643,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":3052,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3643,"record_offset":6804,"parent_record_offset":6696,"ancestor_record_offset":3052,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3643,"record_offset":6860,"parent_record_offset":6832,"ancestor_record_offset":3052,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3643,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":3052,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3643,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3052,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3643,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":3052,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3643,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":3052,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3643,"record_offset":7108,"parent_record_offset":6468,"ancestor_record_offset":3052,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3643,"record_offset":6696,"parent_record_offset":6468,"ancestor_record_offset":3052,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3643,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":3052,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3643,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":3052,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3643,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":3052,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3643,"record_offset":7020,"parent_record_offset":6956,"ancestor_record_offset":3052,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3643,"record_offset":6520,"parent_record_offset":6468,"ancestor_record_offset":3052,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3643,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":3052,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3643,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":3052,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3643,"record_offset":6612,"parent_record_offset":6468,"ancestor_record_offset":3052,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3643,"record_offset":5980,"parent_record_offset":5908,"ancestor_record_offset":3052,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":3052,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":3052,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5260,"parent_record_offset":5232,"ancestor_record_offset":3052,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":3052,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5204,"parent_record_offset":5172,"ancestor_record_offset":3052,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5376,"parent_record_offset":5204,"ancestor_record_offset":3052,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3643,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":3052,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3643,"record_offset":6156,"parent_record_offset":5908,"ancestor_record_offset":3052,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3643,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":3052,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3643,"record_offset":3456,"parent_record_offset":3360,"ancestor_record_offset":3052,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3643,"record_offset":7224,"parent_record_offset":7172,"ancestor_record_offset":3052,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3643,"record_offset":7252,"parent_record_offset":7172,"ancestor_record_offset":3052,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3643,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":3052,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3643,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":3052,"inlinee":6226,"source_line":46,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3643,"record_offset":5952,"parent_record_offset":5908,"ancestor_record_offset":3052,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3643,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":3052,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3643,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":3052,"inlinee":13144,"source_line":160,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3643,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":3052,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3643,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":3052,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3643,"record_offset":4416,"parent_record_offset":3956,"ancestor_record_offset":3052,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3643,"record_offset":5116,"parent_record_offset":4940,"ancestor_record_offset":3052,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3643,"record_offset":3768,"parent_record_offset":3740,"ancestor_record_offset":3052,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3643,"record_offset":6092,"parent_record_offset":6004,"ancestor_record_offset":3052,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3643,"record_offset":5440,"parent_record_offset":5144,"ancestor_record_offset":3052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3643,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":3052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3643,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3052,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3643,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":3052,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3643,"record_offset":4376,"parent_record_offset":4344,"ancestor_record_offset":3052,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3643,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3052,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3643,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":3052,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3643,"record_offset":5840,"parent_record_offset":4904,"ancestor_record_offset":3052,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3643,"record_offset":7744,"parent_record_offset":7700,"ancestor_record_offset":3052,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3643,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":3052,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3643,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3052,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3643,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3643,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":3052,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3643,"record_offset":6036,"parent_record_offset":6004,"ancestor_record_offset":3052,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3643,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":3052,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3643,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":3052,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3643,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3052,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3643,"record_offset":6548,"parent_record_offset":6468,"ancestor_record_offset":3052,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3643,"record_offset":4120,"parent_record_offset":4012,"ancestor_record_offset":3052,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3643,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":3052,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3643,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":3052,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3643,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":3052,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3643,"record_offset":3956,"parent_record_offset":3924,"ancestor_record_offset":3052,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3643,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3052,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3643,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3052,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3643,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3052,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3643,"record_offset":4448,"parent_record_offset":3924,"ancestor_record_offset":3052,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3556,"parent_record_offset":3524,"ancestor_record_offset":3052,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3716,"parent_record_offset":3556,"ancestor_record_offset":3052,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":3052,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":3052,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3052,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":3052,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3643,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":3052,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3643,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":3052,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3643,"record_offset":4260,"parent_record_offset":4012,"ancestor_record_offset":3052,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3643,"record_offset":4232,"parent_record_offset":4012,"ancestor_record_offset":3052,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3643,"record_offset":4064,"parent_record_offset":4012,"ancestor_record_offset":3052,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":4868,"parent_record_offset":4832,"ancestor_record_offset":3052,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3643,"record_offset":4980,"parent_record_offset":4940,"ancestor_record_offset":3052,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":5144,"parent_record_offset":4940,"ancestor_record_offset":3052,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3643,"record_offset":5412,"parent_record_offset":5144,"ancestor_record_offset":3052,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3643,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":3052,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3643,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":3052,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3643,"record_offset":4940,"parent_record_offset":4904,"ancestor_record_offset":3052,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3643,"record_offset":4904,"parent_record_offset":4868,"ancestor_record_offset":3052,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_utils::taffy_mut_style
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_utils.rs
//     evidence={"module":3643,"record_offset":7700,"parent_record_offset":7672,"ancestor_record_offset":3052,"inlinee":84154,"source_line":23,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":3643,"record_offset":7780,"parent_record_offset":7700,"ancestor_record_offset":3052,"inlinee":108725,"source_line":54,"archive":"_pdb/A/modules/03643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A8B5
// 首条位置: {"module":3643,"inlinee":108725}
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
// 完整结构及原始类型记录: toolbar_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_update::toolbar_button_size_and_padding_update
// 信息 ID: 684a2029bc385542741c7867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e0de4993b54e7a90e63505b7: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x15e3c80 -> 0x1b593b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x16620b9 -> 0x1b593b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
// 信息 ID: 554f3748f9347b6287166840
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::toolbar_update::toolbar_button_size_and_padding_update", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::toolbar_update::toolbar_button_size_and_padding_update; module=3643, inline_record=7780; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1>; module=3698, inline_record=5784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
