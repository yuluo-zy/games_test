// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/stairs_signifiers.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7683e1667d1dbb9adae35cff37d426c8aff75965e237e0b8345326893beb6cb2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1251, 1263, 1264
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
// 函数符号: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1251,"record_offset":260,"rva":13471920,"va":5382181040}
// 编译实例 RVAs: 0xcd90b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12372522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8d63543ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382186069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9431529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8171670ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382181220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9431904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8a01470ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382181611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140628e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7014305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e85ff894ff","size":5,"soft_fail":false,"target_va":5375168144,"va":5382182444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7012341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e80b0095ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5382183392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5244450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8def9afff","size":5,"soft_fail":false,"target_va":5376939904,"va":5382184349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5241216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e88006b0ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382185179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5240589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8f308b0ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382184584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-315825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e84f2efbff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382186060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cac5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-183518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e82233fdff","size":5,"soft_fail":false,"target_va":5381998064,"va":5382181577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8e0affdff","size":5,"soft_fail":false,"target_va":5382031392,"va":5382182971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb57c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-147705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e807bffdff","size":5,"soft_fail":false,"target_va":5382035392,"va":5382183092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb5a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-145333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e84bc8fdff","size":5,"soft_fail":false,"target_va":5382036064,"va":5382181392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21410375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e847b24601","size":5,"soft_fail":false,"target_va":5403593968,"va":5382183588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20790688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8a03d3d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382187451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-10876372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e82c0a5aff","size":5,"soft_fail":false,"target_va":5371305056,"va":5382181423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20791986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8b2423d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382186889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20867519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8bf693e01","size":5,"soft_fail":false,"target_va":5403054560,"va":5382187036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20792530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8d2443d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382186617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t20792951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e877463d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382186388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20792209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e891433d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382186730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20793143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e837473d01","size":5,"soft_fail":false,"target_va":5402980304,"va":5382187156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20793286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8c6473d01","size":5,"soft_fail":false,"target_va":5402980400,"va":5382187109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20793182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e85e473d01","size":5,"soft_fail":false,"target_va":5402980320,"va":5382187133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-311829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8eb3dfbff","size":5,"soft_fail":false,"target_va":5381870304,"va":5382182128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-249313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e81f32fcff","size":5,"soft_fail":false,"target_va":5381932864,"va":5382182172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-248350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8e235fcff","size":5,"soft_fail":false,"target_va":5381932832,"va":5382181177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4225751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e82985bfff","size":5,"soft_fail":false,"target_va":5377955632,"va":5382181378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5244898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e81ef8afff","size":5,"soft_fail":false,"target_va":5376938912,"va":5382183805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11166432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e8e062aa00","size":5,"soft_fail":false,"target_va":5393349744,"va":5382183307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11166493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e81d63aa00","size":5,"soft_fail":false,"target_va":5393349872,"va":5382183374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5242213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382181040,"raw_hex":"e89b02b0ff","size":5,"soft_fail":false,"target_va":5376942944,"va":5382185152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":560,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1251,"record_offset":712,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":3280,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":2016,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":796,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":2356,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":2044,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1251,"record_offset":2240,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1251,"record_offset":11044,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1251,"record_offset":11656,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1251,"record_offset":2932,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1251,"record_offset":2840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1251,"record_offset":3492,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1251,"record_offset":924,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1251,"record_offset":1476,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1251,"record_offset":1420,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1251,"record_offset":2960,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":3584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":5628,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1251,"record_offset":1532,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":1336,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1251,"record_offset":5656,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1251,"record_offset":5488,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1251,"record_offset":3336,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59895,"source_line":433,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":9852,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":10040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":10200,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":10428,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":9764,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":10068,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1251,"record_offset":1200,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1251,"record_offset":6256,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1251,"record_offset":11252,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":4060,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":1040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1251,"record_offset":5832,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1251,"record_offset":4672,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":592,"parent_record_offset":560,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1251,"record_offset":1684,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1251,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1251,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1251,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1251,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":836,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":260,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":4152,"parent_record_offset":4092,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1251,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1251,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1251,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1251,"record_offset":11132,"parent_record_offset":11068,"ancestor_record_offset":260,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1251,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1251,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1251,"record_offset":1004,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1251,"record_offset":11564,"parent_record_offset":11276,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1251,"record_offset":11364,"parent_record_offset":11312,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1251,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1251,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1251,"record_offset":5984,"parent_record_offset":5832,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1251,"record_offset":6012,"parent_record_offset":5832,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1251,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":260,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1251,"record_offset":6380,"parent_record_offset":6340,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1251,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1251,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":260,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1251,"record_offset":7764,"parent_record_offset":7564,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1251,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1251,"record_offset":1648,"parent_record_offset":1616,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":4540,"parent_record_offset":4444,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":4596,"parent_record_offset":4444,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":4484,"parent_record_offset":4444,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1251,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1251,"record_offset":11104,"parent_record_offset":11068,"ancestor_record_offset":260,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1251,"record_offset":11068,"parent_record_offset":11044,"ancestor_record_offset":260,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":5872,"parent_record_offset":5832,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":2476,"parent_record_offset":2428,"ancestor_record_offset":260,"inlinee":27288,"source_line":454,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1251,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1251,"record_offset":6544,"parent_record_offset":6380,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1251,"record_offset":7820,"parent_record_offset":7564,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1251,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":260,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1251,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1251,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1251,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1251,"record_offset":2296,"parent_record_offset":2240,"ancestor_record_offset":260,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1251,"record_offset":1292,"parent_record_offset":1256,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1251,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":260,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1251,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1251,"record_offset":892,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":260,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":260,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":7636,"parent_record_offset":7564,"ancestor_record_offset":260,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1251,"record_offset":7700,"parent_record_offset":7660,"ancestor_record_offset":260,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":2392,"parent_record_offset":2356,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":260,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":2548,"parent_record_offset":2504,"ancestor_record_offset":260,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":2504,"parent_record_offset":2428,"ancestor_record_offset":260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$8::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1251,"record_offset":2428,"parent_record_offset":2392,"ancestor_record_offset":260,"inlinee":20602,"source_line":583,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1251,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1251,"record_offset":5568,"parent_record_offset":5488,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1251,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1251,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":260,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1251,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":260,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1251,"record_offset":11312,"parent_record_offset":11276,"ancestor_record_offset":260,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1251,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":260,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":9936,"parent_record_offset":9904,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":260,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1251,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":10324,"parent_record_offset":10224,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":260,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1251,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1251,"record_offset":7188,"parent_record_offset":6812,"ancestor_record_offset":260,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1251,"record_offset":6812,"parent_record_offset":6780,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1251,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1251,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1251,"record_offset":9968,"parent_record_offset":9936,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/stairs_signifiers.rs
//     evidence={"module":1251,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":260,"inlinee":59916,"source_line":203,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":7564,"parent_record_offset":7520,"ancestor_record_offset":260,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":4092,"parent_record_offset":4060,"ancestor_record_offset":260,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":4124,"parent_record_offset":4092,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":4260,"parent_record_offset":4092,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1251,"record_offset":7856,"parent_record_offset":7564,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1251,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":260,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1251,"record_offset":8364,"parent_record_offset":8044,"ancestor_record_offset":260,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1251,"record_offset":4396,"parent_record_offset":4092,"ancestor_record_offset":260,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA0C
// 首条位置: {"module":1251,"inlinee":59916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1251,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":6592,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":6668,"parent_record_offset":6616,"ancestor_record_offset":6592,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":6724,"parent_record_offset":6616,"ancestor_record_offset":6592,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01251.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1251,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":6592,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01251.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_signifiers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_signifiers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_signifiers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
// 信息 ID: 5c19d9c3233206dfdd47cf0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eadd8 -> 0xcd90b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
// 信息 ID: 0d3e524ee90928fb048e0c71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new; module=1251, inline_record=6592; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new; module=1251, inline_record=6924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
