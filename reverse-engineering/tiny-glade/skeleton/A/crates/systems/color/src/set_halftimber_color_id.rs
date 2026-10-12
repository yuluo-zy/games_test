// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/set_halftimber_color_id.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7969e6f83158e9a047171e86a8a66ae3e924f1844daf74c8fd002a0f808f64f5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: set_halftimber_color_id.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3474
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::set_halftimber_color_id::set_halftimber_color_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3474,"record_offset":1372,"rva":27971056,"va":5396680176}
// 编译实例 RVAs: 0x1aacdf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26867202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8fe0966fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5396680749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24050661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e81b0491fe","size":5,"soft_fail":false,"target_va":5372630832,"va":5396681488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21308497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8afdbbafe","size":5,"soft_fail":false,"target_va":5375372576,"va":5396681068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20394883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e87dccc8fe","size":5,"soft_fail":false,"target_va":5376286288,"va":5396681166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19737009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e84fd6d2fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5396681004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc81e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3257195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e86bb33100","size":5,"soft_fail":false,"target_va":5399937504,"va":5396680304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: set_halftimber_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6912971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8cb7b6900","size":5,"soft_fail":false,"target_va":5403593344,"va":5396680368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25375820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8b4cb7cfe","size":5,"soft_fail":false,"target_va":5371305232,"va":5396681047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6297579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8eb176000","size":5,"soft_fail":false,"target_va":5402978880,"va":5396681296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6733287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8e7bd6600","size":5,"soft_fail":false,"target_va":5403414576,"va":5396681284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6297905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e831196000","size":5,"soft_fail":false,"target_va":5402979152,"va":5396681242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6298690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e8421c6000","size":5,"soft_fail":false,"target_va":5402980128,"va":5396681433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14748121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e827f61eff","size":5,"soft_fail":false,"target_va":5381932864,"va":5396680980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21245839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e871d0bbfe","size":5,"soft_fail":false,"target_va":5375434976,"va":5396680810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12260769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396680176,"raw_hex":"e85fea44ff","size":5,"soft_fail":false,"target_va":5384420192,"va":5396680956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396680176,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396680388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3474,"record_offset":3208,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":1556,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":29646,"source_line":544,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2028,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2560,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2836,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3474,"record_offset":1992,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3474,"record_offset":3712,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3474,"record_offset":1752,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3474,"record_offset":3604,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3474,"record_offset":3264,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":27397,"source_line":875,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3474,"record_offset":2708,"parent_record_offset":2676,"ancestor_record_offset":1372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":2676,"parent_record_offset":2632,"ancestor_record_offset":1372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1372,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3474,"record_offset":2632,"parent_record_offset":2588,"ancestor_record_offset":1372,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3474,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1372,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3474,"record_offset":2776,"parent_record_offset":2560,"ancestor_record_offset":1372,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1372,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":1372,"inlinee":29648,"source_line":331,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2588,"parent_record_offset":2560,"ancestor_record_offset":1372,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":1620,"parent_record_offset":1580,"ancestor_record_offset":1372,"inlinee":29647,"source_line":733,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1372,"inlinee":6368,"source_line":717,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1372,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":1372,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3474,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":1372,"inlinee":10427,"source_line":846,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$13::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3474,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1372,"inlinee":23969,"source_line":798,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3474,"record_offset":1820,"parent_record_offset":1780,"ancestor_record_offset":1372,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3474,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":1372,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3474,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":1372,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3474,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1372,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3474,"record_offset":2892,"parent_record_offset":2860,"ancestor_record_offset":1372,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3474,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":1372,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3474,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":1372,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3474,"record_offset":3332,"parent_record_offset":3288,"ancestor_record_offset":1372,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3474,"record_offset":1708,"parent_record_offset":1620,"ancestor_record_offset":1372,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3474,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1372,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3474,"record_offset":3144,"parent_record_offset":2860,"ancestor_record_offset":1372,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3474,"record_offset":2972,"parent_record_offset":2892,"ancestor_record_offset":1372,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3474,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1372,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3474,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":1372,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3474,"record_offset":3432,"parent_record_offset":3388,"ancestor_record_offset":1372,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3474,"record_offset":2740,"parent_record_offset":2632,"ancestor_record_offset":1372,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3474,"record_offset":3668,"parent_record_offset":3632,"ancestor_record_offset":1372,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3474,"record_offset":1904,"parent_record_offset":1876,"ancestor_record_offset":1372,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3474,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":1372,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3474,"record_offset":2212,"parent_record_offset":2100,"ancestor_record_offset":1372,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3474,"record_offset":2404,"parent_record_offset":2100,"ancestor_record_offset":1372,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3474,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1372,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3474,"record_offset":2252,"parent_record_offset":2212,"ancestor_record_offset":1372,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3474,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1372,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3474,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":1372,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3474,"record_offset":3048,"parent_record_offset":2996,"ancestor_record_offset":1372,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3474,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":1372,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3474,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":1372,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3474,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1372,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3474,"record_offset":3476,"parent_record_offset":3288,"ancestor_record_offset":1372,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3474,"record_offset":3540,"parent_record_offset":3288,"ancestor_record_offset":1372,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/03474.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":3474,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":1372,"inlinee":23220,"source_line":91,"archive":"_pdb/A/modules/03474.jsonl.gz"}
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
// 完整结构及原始类型记录: set_halftimber_color_id.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: set_halftimber_color_id.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: set_halftimber_color_id.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>

// ------------------------------------------------------------------
// 函数: system_color::set_halftimber_color_id::set_halftimber_color_id
// 信息 ID: a06e22ec4475ac89872a95f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e1b642a67d8ff99021dbf155: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>)> > @ 0x14991ce -> 0x1aacdf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>)> @ 0x14b3a8a -> 0x1aacdf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
