// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/wall_skirts.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7235a4164a9bd7081644dccd49f7961f61fcf865444babb6836b4105345e9d8c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_skirts.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1226, 1243, 1250, 1264
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
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"PDB_registry_path_only","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_skirts::add_skirts
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1250,"record_offset":260,"rva":13464464,"va":5382173584}
// 编译实例 RVAs: 0xcd7390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13457610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e836a732ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382178277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12364888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8a85343ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382178435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9424110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8123370ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382173801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9425335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8492e70ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382175042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140628e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7008301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8d30f95ff","size":5,"soft_fail":false,"target_va":5375168144,"va":5382176440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5237872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e89013b0ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5382177771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5234153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e81722b0ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382178116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-308188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8244cfbff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382178423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_skirts.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21419271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e807d54601","size":5,"soft_fail":false,"target_va":5403593968,"va":5382174692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20798945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8e15d3d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382179194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20743636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8d4853c01","size":5,"soft_fail":false,"target_va":5402918528,"va":5382174887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-312396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8b43bfbff","size":5,"soft_fail":false,"target_va":5381866128,"va":5382178519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20799853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e86d613d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382179022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20875431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8a7883e01","size":5,"soft_fail":false,"target_va":5403054560,"va":5382179124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20800142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e88e623d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382179005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20800380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e87c633d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382178959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-240623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e81154fcff","size":5,"soft_fail":false,"target_va":5381933120,"va":5382173738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-240861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e82353fcff","size":5,"soft_fail":false,"target_va":5381932864,"va":5382173720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7352717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e873ce8fff","size":5,"soft_fail":false,"target_va":5374821408,"va":5382174120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t11167163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8bb65aa00","size":5,"soft_fail":false,"target_va":5393343808,"va":5382176640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t2243635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8333c2200","size":5,"soft_fail":false,"target_va":5384420192,"va":5382176552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2243708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e87c3c2200","size":5,"soft_fail":false,"target_va":5384420000,"va":5382176287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2244214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8763e2200","size":5,"soft_fail":false,"target_va":5384420752,"va":5382176533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2243753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8a93c2200","size":5,"soft_fail":false,"target_va":5384420368,"va":5382176610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2244235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e88b3e2200","size":5,"soft_fail":false,"target_va":5384420736,"va":5382176496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-4217996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e874a3bfff","size":5,"soft_fail":false,"target_va":5377955840,"va":5382173831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5235033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8a71eb0ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5382174980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5235919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8311bb0ff","size":5,"soft_fail":false,"target_va":5376938912,"va":5382174826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t78268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382173584,"raw_hex":"e8bc310100","size":5,"soft_fail":false,"target_va":5382256592,"va":5382178319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1250,"record_offset":7048,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1250,"record_offset":7388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10715,"source_line":46,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1250,"record_offset":1212,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1416,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":7252,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":824,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1452,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1250,"record_offset":912,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1250,"record_offset":9092,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1250,"record_offset":7196,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1250,"record_offset":6868,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":7104,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1250,"record_offset":2468,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1250,"record_offset":3820,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":1296,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1250,"record_offset":7224,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1250,"record_offset":6840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1250,"record_offset":7076,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59895,"source_line":433,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1250,"record_offset":9372,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8904,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":9176,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1250,"record_offset":3904,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1250,"record_offset":4024,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1250,"record_offset":1388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_skirts::add_skirts::closure$2
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":2788,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59910,"source_line":98,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":7612,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":8080,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59679,"source_line":626,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":1360,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":552,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1250,"record_offset":2620,"parent_record_offset":2520,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1250,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":260,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1250,"record_offset":7548,"parent_record_offset":7416,"ancestor_record_offset":260,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1250,"record_offset":7416,"parent_record_offset":7388,"ancestor_record_offset":260,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1250,"record_offset":7452,"parent_record_offset":7416,"ancestor_record_offset":260,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1250,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":260,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":260,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":260,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1250,"record_offset":2520,"parent_record_offset":2492,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1250,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":260,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1748,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1524,"parent_record_offset":1488,"ancestor_record_offset":260,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":260,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":1624,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":7920,"parent_record_offset":7612,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1250,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":260,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1250,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1250,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1250,"record_offset":2368,"parent_record_offset":2312,"ancestor_record_offset":260,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1250,"record_offset":1804,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1250,"record_offset":1488,"parent_record_offset":1452,"ancestor_record_offset":260,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1250,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1250,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1250,"record_offset":9424,"parent_record_offset":9372,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1250,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1250,"record_offset":8044,"parent_record_offset":7612,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":1568,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":2076,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1250,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":260,"inlinee":11439,"source_line":122,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1250,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":260,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1250,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":24533,"source_line":91,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1250,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":260,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1250,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":260,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1250,"record_offset":1832,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1250,"record_offset":2288,"parent_record_offset":2076,"ancestor_record_offset":260,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1250,"record_offset":852,"parent_record_offset":824,"ancestor_record_offset":260,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1250,"record_offset":7800,"parent_record_offset":7612,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1250,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1250,"record_offset":2408,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1250,"record_offset":2584,"parent_record_offset":2552,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1250,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":260,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3720,"parent_record_offset":3668,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3784,"parent_record_offset":3668,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3132,"parent_record_offset":2868,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1250,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1250,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1250,"record_offset":2168,"parent_record_offset":2132,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1250,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":260,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1250,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":260,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1250,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1250,"record_offset":2752,"parent_record_offset":2716,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1250,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":260,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1250,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1250,"record_offset":880,"parent_record_offset":824,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1250,"record_offset":1028,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1250,"record_offset":972,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1250,"record_offset":2020,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1250,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1250,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":260,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1250,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1250,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1250,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":260,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1250,"record_offset":7672,"parent_record_offset":7612,"ancestor_record_offset":260,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1250,"record_offset":7736,"parent_record_offset":7696,"ancestor_record_offset":260,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1250,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1250,"record_offset":7976,"parent_record_offset":7948,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1250,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":1976,"parent_record_offset":1888,"ancestor_record_offset":260,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":260,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1250,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":260,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1250,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":260,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8988,"parent_record_offset":8956,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":260,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1250,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1250,"record_offset":4224,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1250,"record_offset":2936,"parent_record_offset":2868,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1250,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1250,"record_offset":3668,"parent_record_offset":2788,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1250,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1250,"record_offset":9020,"parent_record_offset":8988,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":260,"inlinee":59600,"source_line":81,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":260,"inlinee":59842,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1250,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":260,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":4196,"parent_record_offset":4124,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":3636,"parent_record_offset":2788,"ancestor_record_offset":260,"inlinee":59887,"source_line":1123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2836,"parent_record_offset":2788,"ancestor_record_offset":260,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":260,"inlinee":59845,"source_line":240,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":260,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":260,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":260,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":260,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1250,"record_offset":8668,"parent_record_offset":8608,"ancestor_record_offset":260,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1250,"record_offset":3164,"parent_record_offset":2868,"ancestor_record_offset":260,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_skirts::add_skirts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8D0
// 首条位置: {"module":1226,"inlinee":59600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1250,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":2104,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1250,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":2104,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1250,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":2104,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1250,"record_offset":2168,"parent_record_offset":2132,"ancestor_record_offset":2104,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9C2
// 首条位置: {"module":1243,"inlinee":59842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_skirts::add_skirts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA06
// 首条位置: {"module":1250,"inlinee":59910}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1250,"record_offset":3668,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":3636,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":59887,"source_line":1123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2836,"parent_record_offset":2788,"ancestor_record_offset":2788,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1250,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2788,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1250,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2788,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1250,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2788,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1250,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":2788,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3720,"parent_record_offset":3668,"ancestor_record_offset":2788,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3784,"parent_record_offset":3668,"ancestor_record_offset":2788,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1250,"record_offset":3132,"parent_record_offset":2868,"ancestor_record_offset":2788,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1250,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2788,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1250,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2788,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1250,"record_offset":3432,"parent_record_offset":3396,"ancestor_record_offset":2788,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":2788,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1250,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":4052,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1250,"record_offset":4224,"parent_record_offset":4124,"ancestor_record_offset":4052,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1250,"record_offset":2936,"parent_record_offset":2868,"ancestor_record_offset":2788,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1250,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":4052,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1250,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":2788,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_skirts.rs
//     evidence={"module":1250,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":2788,"inlinee":59842,"source_line":94,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1250,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":2788,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1250,"record_offset":4196,"parent_record_offset":4124,"ancestor_record_offset":4052,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":2788,"inlinee":59845,"source_line":240,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1250,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":2788,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1250,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":2788,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01250.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1250,"record_offset":3164,"parent_record_offset":2868,"ancestor_record_offset":2788,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01250.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_skirts.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_skirts.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_skirts.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_skirts::add_skirts
// 信息 ID: b9d8b0033d70c9f5b257c348
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ead89 -> 0xcd7390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_skirts::add_skirts::closure$0
// 信息 ID: 2e6a6a6ec440acf63de55390
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_skirts::add_skirts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_skirts::add_skirts::closure$1
// 信息 ID: 7fcc402192ae0ed88a9a18de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::wall_skirts::add_skirts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=3056; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=3552; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=4748; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=5256; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=6068; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=6576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_skirts::add_skirts::closure$2
// 信息 ID: a930bf4d5c03af57f3eee163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::wall_skirts::add_skirts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=2788; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=4052; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=5428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
