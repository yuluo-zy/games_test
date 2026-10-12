// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_arches/profile.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6adcf699ddf18caf935e69f09ce53b0c05147ecbe047815858d8ec1ddba76f85
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: profile.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1231, 1264, 2706
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1231,"record_offset":260,"rva":13341744,"va":5382050864}
// 编译实例 RVAs: 0xcb9430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10353060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e85c0662ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5382053775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9301858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e89e1072ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382051549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9302094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8b20f72ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382051801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14049d3c0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8503740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8443e7eff","size":5,"soft_fail":false,"target_va":5373547456,"va":5382051191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6172612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e83cd0a1ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5382054047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-57733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e87b1effff","size":5,"soft_fail":false,"target_va":5381995056,"va":5382052784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8ee37ffff","size":5,"soft_fail":false,"target_va":5382000496,"va":5382051709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb3bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e82d9fffff","size":5,"soft_fail":false,"target_va":5382028208,"va":5382052990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb3eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e812a6ffff","size":5,"soft_fail":false,"target_va":5382028976,"va":5382051993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8b7adffff","size":5,"soft_fail":false,"target_va":5382031392,"va":5382052452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e85eb4ffff","size":5,"soft_fail":false,"target_va":5382032368,"va":5382051725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e850b7ffff","size":5,"soft_fail":false,"target_va":5382033008,"va":5382051611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb5c70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e803c3ffff","size":5,"soft_fail":false,"target_va":5382036592,"va":5382052200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbb7f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8b61e0000","size":5,"soft_fail":false,"target_va":5382060016,"va":5382052149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t214068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e834440300","size":5,"soft_fail":false,"target_va":5382265392,"va":5382051319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21541769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e889b34801","size":5,"soft_fail":false,"target_va":5403593360,"va":5382051586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20924613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8c5483f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382053526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4526398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8c2eebaff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382053513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20925834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e88a4d3f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382053313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20926140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8bc4e3f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382053199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20925543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8674c3f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382053396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20926668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8cc503f01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382053455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e88f9effff","size":5,"soft_fail":false,"target_va":5382028080,"va":5382053020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8b9a6ffff","size":5,"soft_fail":false,"target_va":5382028352,"va":5382051202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382050864,"raw_hex":"e8c3a6ffff","size":5,"soft_fail":false,"target_va":5382028336,"va":5382051176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":1840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":508,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1231,"record_offset":3132,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1231,"record_offset":7052,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1231,"record_offset":796,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1231,"record_offset":6308,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":5696,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":1244,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":6112,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$106::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1231,"record_offset":6996,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17486,"source_line":35,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1231,"record_offset":2648,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1231,"record_offset":916,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1231,"record_offset":6028,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1231,"record_offset":888,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1231,"record_offset":6000,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1231,"record_offset":1000,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":4096,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":3104,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":2340,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":3376,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24897,"source_line":669,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":2408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1231,"record_offset":3164,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1231,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1231,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1231,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1231,"record_offset":2108,"parent_record_offset":2016,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1231,"record_offset":2812,"parent_record_offset":2720,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1231,"record_offset":2748,"parent_record_offset":2720,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1231,"record_offset":2016,"parent_record_offset":1988,"ancestor_record_offset":260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1231,"record_offset":1988,"parent_record_offset":1960,"ancestor_record_offset":260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1231,"record_offset":2228,"parent_record_offset":1988,"ancestor_record_offset":260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1231,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1231,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":260,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1231,"record_offset":1896,"parent_record_offset":1868,"ancestor_record_offset":260,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1231,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":260,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":3420,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":1960,"parent_record_offset":1924,"ancestor_record_offset":260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":4180,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":260,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1231,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":260,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1231,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":260,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1231,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1231,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1231,"record_offset":3316,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1231,"record_offset":3344,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1231,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1231,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":260,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1231,"record_offset":1300,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1231,"record_offset":6624,"parent_record_offset":6444,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1231,"record_offset":1548,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1231,"record_offset":6248,"parent_record_offset":6112,"ancestor_record_offset":260,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1231,"record_offset":6340,"parent_record_offset":6308,"ancestor_record_offset":260,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1231,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":260,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1231,"record_offset":6372,"parent_record_offset":6340,"ancestor_record_offset":260,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1231,"record_offset":1272,"parent_record_offset":1244,"ancestor_record_offset":260,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1231,"record_offset":1792,"parent_record_offset":1548,"ancestor_record_offset":260,"inlinee":15618,"source_line":109,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1231,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":1180,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":2520,"parent_record_offset":2408,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1231,"record_offset":1328,"parent_record_offset":1300,"ancestor_record_offset":260,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1231,"record_offset":2776,"parent_record_offset":2748,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1231,"record_offset":4544,"parent_record_offset":4448,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1231,"record_offset":4600,"parent_record_offset":4448,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1231,"record_offset":1660,"parent_record_offset":1628,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1231,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1231,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1231,"record_offset":3204,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1231,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1231,"record_offset":2268,"parent_record_offset":1924,"ancestor_record_offset":260,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1231,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1231,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1231,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1231,"record_offset":852,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1231,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1231,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1231,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1231,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1231,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1231,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1231,"record_offset":1120,"parent_record_offset":1056,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1231,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":260,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1231,"record_offset":6564,"parent_record_offset":6484,"ancestor_record_offset":260,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1231,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1231,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":260,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1231,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":260,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1231,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1716,"parent_record_offset":1628,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1231,"record_offset":1368,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1231,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1231,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1231,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":260,"inlinee":59705,"source_line":151,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1231,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":260,"inlinee":59706,"source_line":233,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":260,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":2576,"parent_record_offset":2408,"ancestor_record_offset":260,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":4152,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1231,"record_offset":4276,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1231,"record_offset":4400,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE939
// 首条位置: {"module":1231,"inlinee":59705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1231,"record_offset":1660,"parent_record_offset":1628,"ancestor_record_offset":1576,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1716,"parent_record_offset":1628,"ancestor_record_offset":1576,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1231,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1576,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1231,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":1576,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAB7
// 首条位置: {"module":1264,"inlinee":60087}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1264,"record_offset":6412,"parent_record_offset":5676,"ancestor_record_offset":5676,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1264,"record_offset":6272,"parent_record_offset":5676,"ancestor_record_offset":5676,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1264,"record_offset":5744,"parent_record_offset":5676,"ancestor_record_offset":5676,"inlinee":59713,"source_line":280,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1264,"record_offset":6124,"parent_record_offset":5744,"ancestor_record_offset":5676,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6208,"parent_record_offset":6156,"ancestor_record_offset":5676,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6236,"parent_record_offset":6156,"ancestor_record_offset":5676,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":5676,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1264,"record_offset":6444,"parent_record_offset":6412,"ancestor_record_offset":5676,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1264,"record_offset":6476,"parent_record_offset":6444,"ancestor_record_offset":5676,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1264,"record_offset":6156,"parent_record_offset":5744,"ancestor_record_offset":5676,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5836,"parent_record_offset":5808,"ancestor_record_offset":5676,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5864,"parent_record_offset":5808,"ancestor_record_offset":5676,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1264,"record_offset":5808,"parent_record_offset":5744,"ancestor_record_offset":5676,"inlinee":59709,"source_line":281,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1264,"record_offset":5896,"parent_record_offset":5744,"ancestor_record_offset":5676,"inlinee":59708,"source_line":286,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE93A
// 首条位置: {"module":1231,"inlinee":59706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1231,"record_offset":6740,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1231,"record_offset":6768,"parent_record_offset":6740,"ancestor_record_offset":6712,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1231,"record_offset":6796,"parent_record_offset":6768,"ancestor_record_offset":6712,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01231.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE941
// 首条位置: {"module":1264,"inlinee":59713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1264,"record_offset":6124,"parent_record_offset":5744,"ancestor_record_offset":5744,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1264,"record_offset":6156,"parent_record_offset":5744,"ancestor_record_offset":5744,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1264,"record_offset":5808,"parent_record_offset":5744,"ancestor_record_offset":5744,"inlinee":59709,"source_line":281,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1264,"record_offset":5896,"parent_record_offset":5744,"ancestor_record_offset":5744,"inlinee":59708,"source_line":286,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6208,"parent_record_offset":6156,"ancestor_record_offset":5744,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6236,"parent_record_offset":6156,"ancestor_record_offset":5744,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":5744,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5836,"parent_record_offset":5808,"ancestor_record_offset":5744,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5864,"parent_record_offset":5808,"ancestor_record_offset":5744,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE93D
// 首条位置: {"module":1231,"inlinee":59709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5836,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5864,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE93C
// 首条位置: {"module":1231,"inlinee":59708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1264,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01264.jsonl.gz"}
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
// 完整结构及原始类型记录: profile.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_arches::profile::ArchFunction
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: profile.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: profile.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
// 信息 ID: 19d968aaa1e64d5ec19ac831
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=5744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile
// 信息 ID: 59d4925f30f76705862b85d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8834 -> 0xcb9430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$0
// 信息 ID: 22ffa7430d15c961dbc134cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=1231, inline_record=1576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$1
// 信息 ID: 177e213dfa158e142fb38b5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=5676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure$2
// 信息 ID: b0f6977855f8ad007151c2fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=1231, inline_record=6712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
// 信息 ID: 089ba6840670b4fe497fbcf0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=5808; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=6036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 信息 ID: 4cae84bedbbf6aae37347ad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=5896; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=5952; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$19::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile::closure_env$1> > > >; module=1264, inline_record=6008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
