// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/ui_color_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4067c127ceee716849824d1229fa5cfd3cc814f3ed069bda34fe5b753982d353
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2234, 3476
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_roof::ui_color_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3476,"record_offset":488,"rva":27973216,"va":5396682336}
// 编译实例 RVAs: 0x1aad660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23934461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e803ca92fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396684168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20709752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e888fec3fe","size":5,"soft_fail":false,"target_va":5375974272,"va":5396684019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18753516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e814d8e1fe","size":5,"soft_fail":false,"target_va":5377929600,"va":5396683111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac5650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t97421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e88d7c0100","size":5,"soft_fail":false,"target_va":5396780624,"va":5396683198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8d2860100","size":5,"soft_fail":false,"target_va":5396783360,"va":5396683305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19153176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8e8bedbfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5396683059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-25376658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e86ec87cfe","size":5,"soft_fail":false,"target_va":5371306912,"va":5396683565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6294991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8cf0d6000","size":5,"soft_fail":false,"target_va":5402978944,"va":5396683948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_animation::RoofAnimation::is_being_animated
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12492855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8c95f41ff","size":5,"soft_fail":false,"target_va":5384190032,"va":5396682882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20919430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e87acbc0fe","size":5,"soft_fail":false,"target_va":5375763488,"va":5396682913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::triangles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22622533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8bbcea6fe","size":5,"soft_fail":false,"target_va":5374060560,"va":5396683088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21133350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8da87bdfe","size":5,"soft_fail":false,"target_va":5375550288,"va":5396683633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21132878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8b289bdfe","size":5,"soft_fail":false,"target_va":5375550448,"va":5396683321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20911670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e8cae9c0fe","size":5,"soft_fail":false,"target_va":5375771552,"va":5396683217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20911587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e81deac0fe","size":5,"soft_fail":false,"target_va":5375771664,"va":5396683246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20912007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e879e8c0fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5396683170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20911997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396682336,"raw_hex":"e883e8c0fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5396683272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3476,"record_offset":1512,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3476,"record_offset":1456,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3476,"record_offset":2388,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":852,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3476,"record_offset":1652,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3476,"record_offset":652,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3476,"record_offset":1596,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3476,"record_offset":1764,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3476,"record_offset":2332,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3476,"record_offset":3168,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3476,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":488,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3476,"record_offset":3304,"parent_record_offset":3248,"ancestor_record_offset":488,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3476,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":488,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":488,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":2576,"parent_record_offset":2544,"ancestor_record_offset":488,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3476,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":488,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":2544,"parent_record_offset":2504,"ancestor_record_offset":488,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":488,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":2504,"parent_record_offset":2468,"ancestor_record_offset":488,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3476,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":488,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3476,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":488,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3476,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":488,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3476,"record_offset":2468,"parent_record_offset":2388,"ancestor_record_offset":488,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3476,"record_offset":1360,"parent_record_offset":1276,"ancestor_record_offset":488,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3476,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":488,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3476,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":488,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_table_range
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":1920,"parent_record_offset":1796,"ancestor_record_offset":488,"inlinee":50844,"source_line":61,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":488,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":488,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":488,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::fold
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3476,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":488,"inlinee":50842,"source_line":899,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3476,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":488,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3476,"record_offset":1276,"parent_record_offset":876,"ancestor_record_offset":488,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3476,"record_offset":1164,"parent_record_offset":876,"ancestor_record_offset":488,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3476,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":488,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3476,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":488,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3476,"record_offset":1136,"parent_record_offset":876,"ancestor_record_offset":488,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entity_count
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3476,"record_offset":2268,"parent_record_offset":1796,"ancestor_record_offset":488,"inlinee":50843,"source_line":749,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3476,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":488,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3476,"record_offset":1052,"parent_record_offset":876,"ancestor_record_offset":488,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3476,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":488,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3476,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":488,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3476,"record_offset":2152,"parent_record_offset":1920,"ancestor_record_offset":488,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3476,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":488,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3476,"record_offset":1976,"parent_record_offset":1920,"ancestor_record_offset":488,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3476,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":488,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3476,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":488,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3476,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":488,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3476,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":488,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3476,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":488,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3476,"record_offset":964,"parent_record_offset":928,"ancestor_record_offset":488,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3476,"record_offset":2608,"parent_record_offset":2504,"ancestor_record_offset":488,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3476,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":488,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3476,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":488,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3476,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":488,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3476,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":488,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3476,"record_offset":992,"parent_record_offset":928,"ancestor_record_offset":488,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3476,"record_offset":928,"parent_record_offset":876,"ancestor_record_offset":488,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_roof::ui_color_roof::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13CD5
// 首条位置: {"module":2234,"inlinee":81109}
// 编译实例 RVAs: 0x9afba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e8a7aba2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378866132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e829aba2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378866274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24111907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e823eb6f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378866232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11511626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e84aa7af00","size":5,"soft_fail":false,"target_va":5390377808,"va":5378866177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32088,"parent_record_offset":32056,"ancestor_record_offset":32056,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2234,"record_offset":32232,"parent_record_offset":32056,"ancestor_record_offset":32056,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2234,"record_offset":32316,"parent_record_offset":32056,"ancestor_record_offset":32056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32140,"parent_record_offset":32116,"ancestor_record_offset":32056,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32164,"parent_record_offset":32140,"ancestor_record_offset":32056,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32388,"parent_record_offset":32364,"ancestor_record_offset":32056,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32116,"parent_record_offset":32088,"ancestor_record_offset":32056,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2234,"record_offset":32364,"parent_record_offset":32340,"ancestor_record_offset":32056,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2234,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":32056,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2234,"record_offset":32256,"parent_record_offset":32232,"ancestor_record_offset":32056,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2234,"record_offset":32280,"parent_record_offset":32256,"ancestor_record_offset":32056,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2234,"record_offset":32188,"parent_record_offset":32164,"ancestor_record_offset":32056,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02234.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>

// ------------------------------------------------------------------
// 函数: system_color::ui_color_roof::ui_color_roof
// 信息 ID: c4baf3579ef9573c3112d68a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ecf178c7ddac65c973c04d1b: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)> > @ 0x14a005b -> 0x1aad660 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)> @ 0x14aaf29 -> 0x1aad660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_roof::ui_color_roof::closure$0
// 信息 ID: e571776de58db1f9b41c45f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_roof::ui_color_roof", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_roof::ui_color_roof::closure_env$0 <- core::ops::function::FnOnce::call_once<system_color::ui_color_roof::ui_color_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_roof::ui_color_roof::closure_env$0 <- core::ptr::drop_in_place<system_color::ui_color_roof::ui_color_roof::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_roof::ui_color_roof::closure_env$0 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_roof::ui_color_roof::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_roof::ui_color_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=2234, inline_record=32056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
