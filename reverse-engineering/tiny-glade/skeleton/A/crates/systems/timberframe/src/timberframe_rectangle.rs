// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/timberframe/src/timberframe_rectangle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9b2480b744afe8258003d6cdd49ab6625a9a708f3601fd4869e1fb7da8479996
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: timberframe_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1957
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
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use timberframe; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_rectangle::construct_timberframe_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1957,"record_offset":248,"rva":19830464,"va":5388539584}
// 编译实例 RVAs: 0x12e96c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18731026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8ee2fe2fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5388544573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16846244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e85cf2fefe","size":5,"soft_fail":false,"target_va":5371700720,"va":5388546959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15790167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8a90f0fff","size":5,"soft_fail":false,"target_va":5372749696,"va":5388539858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15791977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e897080fff","size":5,"soft_fail":false,"target_va":5372749712,"va":5388541684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12666772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e86cb83eff","size":5,"soft_fail":false,"target_va":5375881440,"va":5388548207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11599888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8f0ff4eff","size":5,"soft_fail":false,"target_va":5376944000,"va":5388543883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6671316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e82c349aff","size":5,"soft_fail":false,"target_va":5381870240,"va":5388541551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6276465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e88f3aa0ff","size":5,"soft_fail":false,"target_va":5382265392,"va":5388541852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e6350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8e3caffff","size":5,"soft_fail":false,"target_va":5388526416,"va":5388540008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6969811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8d3596a00","size":5,"soft_fail":false,"target_va":5395509552,"va":5388539736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f400
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6968648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e848556a00","size":5,"soft_fail":false,"target_va":5395510272,"va":5388541619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6970678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8365d6a00","size":5,"soft_fail":false,"target_va":5395510992,"va":5388540309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15053533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8ddb2e500","size":5,"soft_fail":false,"target_va":5403593344,"va":5388539806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15051989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8d5ace500","size":5,"soft_fail":false,"target_va":5403594096,"va":5388542102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14431562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e84a35dc00","size":5,"soft_fail":false,"target_va":5402978144,"va":5388546577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::downgrade::panic_cold_display::h6d302680404e26f3 (.llvm.17036871241626899607)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14985843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e873aae400","size":5,"soft_fail":false,"target_va":5403532272,"va":5388546424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t14432394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e88a38dc00","size":5,"soft_fail":false,"target_va":5402978880,"va":5388546481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14782678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8d690e100","size":5,"soft_fail":false,"target_va":5403329152,"va":5388546469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13955827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e80d0d2bff","size":5,"soft_fail":false,"target_va":5374588736,"va":5388544558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12779263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e801013dff","size":5,"soft_fail":false,"target_va":5375765280,"va":5388544538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12778555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8c5033dff","size":5,"soft_fail":false,"target_va":5375765952,"va":5388544502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13721948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8a49e2eff","size":5,"soft_fail":false,"target_va":5374821408,"va":5388543351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11192522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8363755ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5388546293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14428759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8572adc00","size":5,"soft_fail":false,"target_va":5402968992,"va":5388540228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14424378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e83a19dc00","size":5,"soft_fail":false,"target_va":5402970800,"va":5388546417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10590546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8ae665eff","size":5,"soft_fail":false,"target_va":5377954064,"va":5388544605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10586094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e812785eff","size":5,"soft_fail":false,"target_va":5377954656,"va":5388540745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10589913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e827695eff","size":5,"soft_fail":false,"target_va":5377955632,"va":5388545540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat3
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10585349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e8fb7a5eff","size":5,"soft_fail":false,"target_va":5377955488,"va":5388540832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10590072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e888685eff","size":5,"soft_fail":false,"target_va":5377955424,"va":5388545491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-11602587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388539584,"raw_hex":"e865f54eff","size":5,"soft_fail":false,"target_va":5376941312,"va":5388543894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5388539584,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5388539826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1957,"record_offset":1368,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1957,"record_offset":2288,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1957,"record_offset":2916,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1957,"record_offset":13516,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1957,"record_offset":4240,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1957,"record_offset":13544,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1957,"record_offset":7100,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1957,"record_offset":9820,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1957,"record_offset":5480,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":1728,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":528,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":824,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1957,"record_offset":1424,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1957,"record_offset":4780,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1957,"record_offset":3240,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1957,"record_offset":584,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1957,"record_offset":860,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14712,"source_line":117,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1957,"record_offset":1792,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1957,"record_offset":3320,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1957,"record_offset":8108,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1957,"record_offset":8020,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":5268,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":9972,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":7684,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":9884,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":7932,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1957,"record_offset":4864,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1957,"record_offset":5112,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1957,"record_offset":13572,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1957,"record_offset":13432,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1957,"record_offset":5452,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_rectangle.rs
//     evidence={"module":1957,"record_offset":2828,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":77253,"source_line":40,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1957,"record_offset":3676,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":248,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":248,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":960,"parent_record_offset":928,"ancestor_record_offset":248,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1957,"record_offset":4484,"parent_record_offset":4380,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1957,"record_offset":900,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1957,"record_offset":4412,"parent_record_offset":4380,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1957,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1957,"record_offset":1084,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":11196,"source_line":1613,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1957,"record_offset":1136,"parent_record_offset":1084,"ancestor_record_offset":248,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1957,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1957,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":248,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1957,"record_offset":2352,"parent_record_offset":2312,"ancestor_record_offset":248,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1957,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":248,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1957,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":248,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1957,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":248,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1957,"record_offset":652,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1957,"record_offset":4148,"parent_record_offset":3700,"ancestor_record_offset":248,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1957,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":248,"inlinee":8944,"source_line":21,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1957,"record_offset":3948,"parent_record_offset":3896,"ancestor_record_offset":248,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1957,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":248,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":1957,"record_offset":1192,"parent_record_offset":1084,"ancestor_record_offset":248,"inlinee":8943,"source_line":216,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1957,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":248,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1957,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":248,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1957,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1957,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":248,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1957,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":248,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1957,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":248,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1957,"record_offset":5048,"parent_record_offset":4888,"ancestor_record_offset":248,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1957,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":248,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1957,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":248,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1957,"record_offset":2424,"parent_record_offset":2376,"ancestor_record_offset":248,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1957,"record_offset":4448,"parent_record_offset":4412,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1957,"record_offset":8264,"parent_record_offset":8132,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1957,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":248,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1957,"record_offset":7960,"parent_record_offset":7932,"ancestor_record_offset":248,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1957,"record_offset":5324,"parent_record_offset":5292,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":2084,"parent_record_offset":1792,"ancestor_record_offset":248,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":248,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":3060,"parent_record_offset":2980,"ancestor_record_offset":248,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":1868,"parent_record_offset":1792,"ancestor_record_offset":248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1957,"record_offset":2124,"parent_record_offset":1792,"ancestor_record_offset":248,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1957,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1957,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1957,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1957,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":248,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1957,"record_offset":3284,"parent_record_offset":3240,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1957,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":248,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1957,"record_offset":1992,"parent_record_offset":1904,"ancestor_record_offset":248,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1957,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":248,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1957,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":248,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1957,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":248,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1957,"record_offset":2488,"parent_record_offset":2376,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1957,"record_offset":2672,"parent_record_offset":2376,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1957,"record_offset":2052,"parent_record_offset":1792,"ancestor_record_offset":248,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1957,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":248,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1957,"record_offset":2520,"parent_record_offset":2488,"ancestor_record_offset":248,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1957,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1957,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1957,"record_offset":3136,"parent_record_offset":3084,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1957,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":248,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":248,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":248,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":1304,"parent_record_offset":1084,"ancestor_record_offset":248,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":1248,"parent_record_offset":1084,"ancestor_record_offset":248,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":248,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1957,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":248,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1957,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":248,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1957,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":248,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1957,"record_offset":13628,"parent_record_offset":13600,"ancestor_record_offset":248,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1957,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":248,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1957,"record_offset":3444,"parent_record_offset":3344,"ancestor_record_offset":248,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1957,"record_offset":3740,"parent_record_offset":3700,"ancestor_record_offset":248,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1957,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":248,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1957,"record_offset":3804,"parent_record_offset":3740,"ancestor_record_offset":248,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1957,"record_offset":3896,"parent_record_offset":3700,"ancestor_record_offset":248,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1957,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":248,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1957,"record_offset":8196,"parent_record_offset":8164,"ancestor_record_offset":248,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1957,"record_offset":8164,"parent_record_offset":8132,"ancestor_record_offset":248,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1957,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":248,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":5380,"parent_record_offset":5292,"ancestor_record_offset":248,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":10036,"parent_record_offset":10000,"ancestor_record_offset":248,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":248,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1957,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1957,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":248,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1957,"record_offset":1904,"parent_record_offset":1792,"ancestor_record_offset":248,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1957,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":248,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1957,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":248,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01957.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12DC5
// 首条位置: {"module":1957,"inlinee":77253}
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
// 完整结构及原始类型记录: timberframe_rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: timberframe_rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: timberframe_rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>", "bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle
// 信息 ID: 0edcfe10feaac39f7cabbf62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9731e58a2bea5a43fffb9ed5: void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> @ 0x8ec234 -> 0x12e96c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> > @ 0x12ed3d6 -> 0x12e96c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 信息 ID: d9e6d049941ffe9f4c9ff815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "timberframe::timberframe_rectangle::construct_timberframe_rectangle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=1957, inline_record=2828; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=1957, inline_record=4268; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=1957, inline_record=6528; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=1957, inline_record=9124; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=1957, inline_record=11184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
