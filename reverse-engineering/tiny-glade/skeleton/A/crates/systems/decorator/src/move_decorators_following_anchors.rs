// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/move_decorators_following_anchors.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6a7f166fc4611bafc2dc698eaaa49d328038516a5cd7f1b217edb8d74fe94fdd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: move_decorators_following_anchors.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 385
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
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":385,"record_offset":492,"rva":5255632,"va":5373964752}
// 编译实例 RVAs: 0x5031d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4351814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8ba98bdff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373965329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t747814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e826690b00","size":5,"soft_fail":false,"target_va":5374712880,"va":5373965061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8289863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8477e7e00","size":5,"soft_fail":false,"target_va":5382255968,"va":5373966100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d07140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8403862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8963b8000","size":5,"soft_fail":false,"target_va":5382369600,"va":5373965733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29012878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e88eb3ba01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373966269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29013109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e875b4ba01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373966230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29012525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e82db2ba01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373966414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_removed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t755449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8f9860b00","size":5,"soft_fail":false,"target_va":5374720464,"va":5373965010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7967650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8a2937900","size":5,"soft_fail":false,"target_va":5381933344,"va":5373965689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7966972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8fc907900","size":5,"soft_fail":false,"target_va":5381933056,"va":5373966079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2973894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373964752,"raw_hex":"e8c6602d00","size":5,"soft_fail":false,"target_va":5376939952,"va":5373966053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":385,"record_offset":1656,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":385,"record_offset":704,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":385,"record_offset":1248,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":385,"record_offset":6024,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":2396,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":2340,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":3508,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":3864,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_approx_coord_from_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":385,"record_offset":4488,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":21778,"source_line":566,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":385,"record_offset":5560,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":21780,"source_line":664,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":385,"record_offset":5996,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":21749,"source_line":106,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":385,"record_offset":5620,"parent_record_offset":5560,"ancestor_record_offset":492,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":385,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":492,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":385,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":492,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":385,"record_offset":768,"parent_record_offset":728,"ancestor_record_offset":492,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":385,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":492,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":385,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":492,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":385,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":492,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":385,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":492,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":385,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":492,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":385,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":492,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":385,"record_offset":2048,"parent_record_offset":2016,"ancestor_record_offset":492,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":385,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":492,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":385,"record_offset":2112,"parent_record_offset":2076,"ancestor_record_offset":492,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":385,"record_offset":5264,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":385,"record_offset":5236,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":385,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":492,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":385,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":492,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":385,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":492,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":385,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":492,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":385,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":492,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":385,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":492,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":385,"record_offset":4660,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":385,"record_offset":4724,"parent_record_offset":4692,"ancestor_record_offset":492,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":385,"record_offset":836,"parent_record_offset":792,"ancestor_record_offset":492,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":385,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":492,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":385,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":492,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":385,"record_offset":5376,"parent_record_offset":5320,"ancestor_record_offset":492,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":385,"record_offset":4964,"parent_record_offset":4932,"ancestor_record_offset":492,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":385,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":492,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":385,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":492,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":385,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":492,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":385,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":492,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":385,"record_offset":4860,"parent_record_offset":4752,"ancestor_record_offset":492,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":385,"record_offset":1596,"parent_record_offset":1248,"ancestor_record_offset":492,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":385,"record_offset":1416,"parent_record_offset":1336,"ancestor_record_offset":492,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":385,"record_offset":2232,"parent_record_offset":1872,"ancestor_record_offset":492,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":385,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":492,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":385,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":492,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":385,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":492,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":385,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":492,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":385,"record_offset":4824,"parent_record_offset":4780,"ancestor_record_offset":492,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":385,"record_offset":900,"parent_record_offset":792,"ancestor_record_offset":492,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":385,"record_offset":1092,"parent_record_offset":792,"ancestor_record_offset":492,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":385,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":492,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":385,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":492,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":385,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":492,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":385,"record_offset":940,"parent_record_offset":900,"ancestor_record_offset":492,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":385,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":492,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":385,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":492,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":385,"record_offset":1492,"parent_record_offset":1440,"ancestor_record_offset":492,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":385,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":492,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":385,"record_offset":4904,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":385,"record_offset":4996,"parent_record_offset":4932,"ancestor_record_offset":492,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":385,"record_offset":5296,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":385,"record_offset":4932,"parent_record_offset":4904,"ancestor_record_offset":492,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":385,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":492,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":385,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":492,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":492,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":492,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":492,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":492,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":3132,"parent_record_offset":2912,"ancestor_record_offset":492,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":492,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":385,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":492,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3020,"parent_record_offset":2912,"ancestor_record_offset":492,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":492,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2148,"parent_record_offset":1872,"ancestor_record_offset":492,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3340,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":492,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":1872,"parent_record_offset":1840,"ancestor_record_offset":492,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":492,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":492,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2500,"parent_record_offset":2444,"ancestor_record_offset":492,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":492,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":492,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2552,"parent_record_offset":2524,"ancestor_record_offset":492,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":492,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":385,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":492,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":385,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":492,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":385,"record_offset":1984,"parent_record_offset":1920,"ancestor_record_offset":492,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":385,"record_offset":1920,"parent_record_offset":1872,"ancestor_record_offset":492,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2596,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":492,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":385,"record_offset":2912,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":385,"record_offset":2624,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":385,"record_offset":2016,"parent_record_offset":1872,"ancestor_record_offset":492,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":385,"record_offset":2708,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":385,"record_offset":3248,"parent_record_offset":2552,"ancestor_record_offset":492,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":385,"record_offset":2076,"parent_record_offset":2016,"ancestor_record_offset":492,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":385,"record_offset":4632,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":385,"record_offset":4692,"parent_record_offset":4660,"ancestor_record_offset":492,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":492,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":385,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":492,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":385,"record_offset":5028,"parent_record_offset":4996,"ancestor_record_offset":492,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/move_decorators_following_anchors.rs
//     evidence={"module":385,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":492,"inlinee":21783,"source_line":44,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":385,"record_offset":5592,"parent_record_offset":5560,"ancestor_record_offset":492,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":385,"record_offset":5716,"parent_record_offset":5560,"ancestor_record_offset":492,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":385,"record_offset":5412,"parent_record_offset":4488,"ancestor_record_offset":492,"inlinee":21779,"source_line":1056,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":385,"record_offset":5840,"parent_record_offset":5560,"ancestor_record_offset":492,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5517
// 首条位置: {"module":385,"inlinee":21783}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":385,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00385.jsonl.gz"}
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
// 完整结构及原始类型记录: move_decorators_following_anchors.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: move_decorators_following_anchors.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: move_decorators_following_anchors.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors
// 信息 ID: 85f26ee13e37be6426aa8653
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0cdbe14b43a4ff051f31c4d2: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)> > @ 0xc84d1 -> 0x5031d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)> @ 0x359f9f -> 0x5031d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
// 信息 ID: 0e66bcb9d6442ecb0b7d9cc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::move_decorators_following_anchors::move_decorators_following_anchors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=385, inline_record=6048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
