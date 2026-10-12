// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/remove_ivy_under_windows.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2284d52197525d15f1ae4428efb1926ba2b30107c3249ffca5eddb6a81f0ea31
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: remove_ivy_under_windows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3855
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3855,"record_offset":256,"rva":29687152,"va":5398396272}
// 编译实例 RVAs: 0x1c4fd70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28783183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e8b1cd48fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398396698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23824722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e8ae7694fe","size":5,"soft_fail":false,"target_va":5374572480,"va":5398397197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23679830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e8aaac96fe","size":5,"soft_fail":false,"target_va":5374716832,"va":5398396657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23187436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e814309efe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398397207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23183121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e8ef409efe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398396348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: remove_ivy_under_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16030227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e8ed650bff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398397230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4581383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e807e84500","size":5,"soft_fail":false,"target_va":5402978944,"va":5398397556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16526848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e800d203ff","size":5,"soft_fail":false,"target_va":5381870304,"va":5398397147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_pruner::remove_ivy_at_location
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23583874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398396272,"raw_hex":"e87e2398fe","size":5,"soft_fail":false,"target_va":5374813504,"va":5398397373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3855,"record_offset":488,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3855,"record_offset":1020,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3855,"record_offset":1332,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3855,"record_offset":3596,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3855,"record_offset":432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3855,"record_offset":3508,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3855,"record_offset":3452,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3855,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3855,"record_offset":552,"parent_record_offset":512,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3855,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3855,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3855,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3855,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3855,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3855,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3855,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3855,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":256,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3855,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":256,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3855,"record_offset":616,"parent_record_offset":576,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3855,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3855,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":256,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3855,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":256,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3855,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":256,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3855,"record_offset":1152,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3855,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":256,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3855,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3855,"record_offset":680,"parent_record_offset":576,"ancestor_record_offset":256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3855,"record_offset":864,"parent_record_offset":576,"ancestor_record_offset":256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3855,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":256,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3855,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":256,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3855,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3855,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3855,"record_offset":1228,"parent_record_offset":1176,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3855,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":256,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3855,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":256,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3855,"record_offset":3632,"parent_record_offset":3596,"ancestor_record_offset":256,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3855,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":256,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3855,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":256,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":256,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":256,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":2120,"parent_record_offset":1900,"ancestor_record_offset":256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":256,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":256,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2008,"parent_record_offset":1900,"ancestor_record_offset":256,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":256,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2328,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":256,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":256,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1492,"parent_record_offset":1432,"ancestor_record_offset":256,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":256,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":256,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":256,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1868,"parent_record_offset":1752,"ancestor_record_offset":256,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1752,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1584,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1900,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":1612,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":1696,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":2236,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3855,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":256,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/remove_ivy_under_windows.rs
//     evidence={"module":3855,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":256,"inlinee":111837,"source_line":28,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/remove_ivy_under_windows.rs
//     evidence={"module":3855,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":256,"inlinee":111838,"source_line":58,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3855,"record_offset":4104,"parent_record_offset":4068,"ancestor_record_offset":256,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4DD
// 首条位置: {"module":3855,"inlinee":111837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3855,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":1360,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3855,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1360,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1360,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1360,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3855,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1360,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3855,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1360,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1360,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1360,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3855,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1360,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3855,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1360,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":1360,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1360,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":2120,"parent_record_offset":1900,"ancestor_record_offset":1360,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":1360,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1360,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2008,"parent_record_offset":1900,"ancestor_record_offset":1360,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1360,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2328,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1360,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1360,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1492,"parent_record_offset":1432,"ancestor_record_offset":1360,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1360,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":1360,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":1360,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1868,"parent_record_offset":1752,"ancestor_record_offset":1360,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":1752,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1584,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":1900,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":1612,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":1696,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":2236,"parent_record_offset":1540,"ancestor_record_offset":1360,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4DE
// 首条位置: {"module":3855,"inlinee":111838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3855,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3855,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":2420,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":2420,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3855,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2420,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3855,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":2420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3855,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2420,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2420,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3855,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":2420,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3855,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":2420,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3855,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2420,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":2420,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":2420,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":3180,"parent_record_offset":2960,"ancestor_record_offset":2420,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3855,"record_offset":2524,"parent_record_offset":2492,"ancestor_record_offset":2420,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":2420,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":3068,"parent_record_offset":2960,"ancestor_record_offset":2420,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2420,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":3388,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2420,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2420,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2552,"parent_record_offset":2492,"ancestor_record_offset":2420,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2420,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":2420,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2420,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":2928,"parent_record_offset":2812,"ancestor_record_offset":2420,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3855,"record_offset":2812,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2644,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3855,"record_offset":2960,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":2672,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":2756,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03855.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3855,"record_offset":3296,"parent_record_offset":2600,"ancestor_record_offset":2420,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03855.jsonl.gz"}
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
// 完整结构及原始类型记录: remove_ivy_under_windows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: remove_ivy_under_windows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: remove_ivy_under_windows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>

// ------------------------------------------------------------------
// 函数: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows
// 信息 ID: 7c1bf0d93808c74525ccc5b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] de197b2f1260d6ed73b18154: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> > @ 0x17d10dd -> 0x1c4fd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> @ 0x17f4089 -> 0x1c4fd70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$0
// 信息 ID: 66c93d9e10e3fc00be71440b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::remove_ivy_under_windows::remove_ivy_under_windows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::remove_ivy_under_windows::remove_ivy_under_windows; module=3855, inline_record=1360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::remove_ivy_under_windows::remove_ivy_under_windows::closure$1
// 信息 ID: 456f3d597ddff2d9611a5e66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::remove_ivy_under_windows::remove_ivy_under_windows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::remove_ivy_under_windows::remove_ivy_under_windows; module=3855, inline_record=2420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
