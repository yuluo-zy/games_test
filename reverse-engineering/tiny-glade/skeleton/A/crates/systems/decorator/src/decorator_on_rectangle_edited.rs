// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_on_rectangle_edited.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4ee6764d1ffcf52287ca448776a3ebda66a54d65babed2000cc341e1890a8227
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_on_rectangle_edited.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 384
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":384,"record_offset":256,"rva":5253792,"va":5373962912}
// 编译实例 RVAs: 0x502aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5243762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e88efcafff","size":5,"soft_fail":false,"target_va":5368720672,"va":5373964429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4350316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8949ebdff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373963831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t749215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e89f6e0b00","size":5,"soft_fail":false,"target_va":5374712640,"va":5373963420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2979928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e858782d00","size":5,"soft_fail":false,"target_va":5376944000,"va":5373964067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d00020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8377314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8e2d37f00","size":5,"soft_fail":false,"target_va":5382340640,"va":5373963321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29014317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e82db9ba01","size":5,"soft_fail":false,"target_va":5402978912,"va":5373964590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29014273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e801b9ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373964602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29014521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8f9b9ba01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373964626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t606995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e813430900","size":5,"soft_fail":false,"target_va":5374570992,"va":5373963992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t749612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e82c700b00","size":5,"soft_fail":false,"target_va":5374712960,"va":5373963343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t97696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8a07d0100","size":5,"soft_fail":false,"target_va":5374061072,"va":5373963371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1322577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8512e1400","size":5,"soft_fail":false,"target_va":5375285936,"va":5373963354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1472617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e869781600","size":5,"soft_fail":false,"target_va":5375436640,"va":5373964018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3997878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8b6003d00","size":5,"soft_fail":false,"target_va":5377961984,"va":5373964101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3991590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e826e83c00","size":5,"soft_fail":false,"target_va":5377955632,"va":5373964037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3991988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e8b4e93c00","size":5,"soft_fail":false,"target_va":5377955440,"va":5373963447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3991306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962912,"raw_hex":"e80ae73c00","size":5,"soft_fail":false,"target_va":5377955424,"va":5373964113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":384,"record_offset":1276,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":384,"record_offset":432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":384,"record_offset":1032,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_on_rectangle_edited.rs
//     evidence={"module":384,"record_offset":1392,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21766,"source_line":35,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":384,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":384,"record_offset":552,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":384,"record_offset":516,"parent_record_offset":468,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":384,"record_offset":1072,"parent_record_offset":1032,"ancestor_record_offset":256,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":384,"record_offset":468,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":384,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":256,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3920,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3864,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3836,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3892,"parent_record_offset":3804,"ancestor_record_offset":256,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":384,"record_offset":4096,"parent_record_offset":4068,"ancestor_record_offset":256,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":384,"record_offset":4124,"parent_record_offset":4068,"ancestor_record_offset":256,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":384,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":384,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":384,"record_offset":612,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":384,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":384,"record_offset":3952,"parent_record_offset":3772,"ancestor_record_offset":256,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":384,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":256,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":384,"record_offset":1144,"parent_record_offset":1112,"ancestor_record_offset":256,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":384,"record_offset":1200,"parent_record_offset":1112,"ancestor_record_offset":256,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":384,"record_offset":1580,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":384,"record_offset":1232,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":384,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":384,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":384,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":384,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":384,"record_offset":644,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":384,"record_offset":744,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":384,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":256,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":384,"record_offset":684,"parent_record_offset":644,"ancestor_record_offset":256,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":384,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":384,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":256,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine2.rs
//     evidence={"module":384,"record_offset":3748,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":21768,"source_line":255,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":384,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":256,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":384,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":256,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":4044,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":256,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":3988,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":384,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":256,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2768,"parent_record_offset":2732,"ancestor_record_offset":256,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":256,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":3480,"parent_record_offset":3260,"ancestor_record_offset":256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2068,"parent_record_offset":2032,"ancestor_record_offset":256,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":256,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":256,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3368,"parent_record_offset":3260,"ancestor_record_offset":256,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":256,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2452,"parent_record_offset":2144,"ancestor_record_offset":256,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3688,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1828,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2144,"parent_record_offset":2104,"ancestor_record_offset":256,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":256,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":256,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2852,"parent_record_offset":2792,"ancestor_record_offset":256,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":256,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1800,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":256,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":256,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":256,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2256,"parent_record_offset":2224,"ancestor_record_offset":256,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":3228,"parent_record_offset":3112,"ancestor_record_offset":256,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2192,"parent_record_offset":2144,"ancestor_record_offset":256,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2944,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2104,"parent_record_offset":2068,"ancestor_record_offset":256,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3260,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":2972,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":1884,"parent_record_offset":1828,"ancestor_record_offset":256,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":3056,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":3596,"parent_record_offset":2900,"ancestor_record_offset":256,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":1940,"parent_record_offset":1884,"ancestor_record_offset":256,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2732,"parent_record_offset":2700,"ancestor_record_offset":256,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":256,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$37::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":1652,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":17308,"source_line":2193,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2032,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2700,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":384,"record_offset":4160,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5506
// 首条位置: {"module":384,"inlinee":21766}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":384,"record_offset":1580,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine2.rs
//     evidence={"module":384,"record_offset":3748,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":21768,"source_line":255,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":4044,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":3988,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$37::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":1652,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":17308,"source_line":2193,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2032,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2700,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":384,"record_offset":4160,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":384,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":1392,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":1392,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":1392,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":384,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":1392,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3920,"parent_record_offset":3804,"ancestor_record_offset":1392,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3864,"parent_record_offset":3804,"ancestor_record_offset":1392,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3836,"parent_record_offset":3804,"ancestor_record_offset":1392,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":384,"record_offset":3892,"parent_record_offset":3804,"ancestor_record_offset":1392,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":384,"record_offset":4096,"parent_record_offset":4068,"ancestor_record_offset":1392,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":384,"record_offset":4124,"parent_record_offset":4068,"ancestor_record_offset":1392,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":384,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1392,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":384,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":1392,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":384,"record_offset":3952,"parent_record_offset":3772,"ancestor_record_offset":1392,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":384,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":1392,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":384,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1392,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":384,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1392,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":384,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1392,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":384,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":1392,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":384,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":1392,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":384,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":1392,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":384,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":1392,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":384,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":1392,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2768,"parent_record_offset":2732,"ancestor_record_offset":1392,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1392,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1392,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":3480,"parent_record_offset":3260,"ancestor_record_offset":1392,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2068,"parent_record_offset":2032,"ancestor_record_offset":1392,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":384,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":1392,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":1392,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3368,"parent_record_offset":3260,"ancestor_record_offset":1392,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":1392,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2452,"parent_record_offset":2144,"ancestor_record_offset":1392,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3688,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1828,"parent_record_offset":1764,"ancestor_record_offset":1392,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2144,"parent_record_offset":2104,"ancestor_record_offset":1392,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":1392,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":1392,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2852,"parent_record_offset":2792,"ancestor_record_offset":1392,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":1392,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1800,"parent_record_offset":1764,"ancestor_record_offset":1392,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":1392,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":1392,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":1392,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2256,"parent_record_offset":2224,"ancestor_record_offset":1392,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":3228,"parent_record_offset":3112,"ancestor_record_offset":1392,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":384,"record_offset":2192,"parent_record_offset":2144,"ancestor_record_offset":1392,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2944,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":2104,"parent_record_offset":2068,"ancestor_record_offset":1392,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":384,"record_offset":3260,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":2972,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":1884,"parent_record_offset":1828,"ancestor_record_offset":1392,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":3056,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":3596,"parent_record_offset":2900,"ancestor_record_offset":1392,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":384,"record_offset":1940,"parent_record_offset":1884,"ancestor_record_offset":1392,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":2732,"parent_record_offset":2700,"ancestor_record_offset":1392,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":384,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":1392,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00384.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_on_rectangle_edited.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_on_rectangle_edited.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_on_rectangle_edited.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited
// 信息 ID: e1ceaed45e1d83c3cc979ca0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fede2b2a3d9729cf5a4d5588: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0xb436e -> 0x502aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> @ 0x2b1da5 -> 0x502aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
// 信息 ID: 2bc338f0267ff7a348f745a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited; module=384, inline_record=1392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
