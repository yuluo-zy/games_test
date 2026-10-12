// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/clear_canvas.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9b8d3c0e99b6540d0ebac2dc7e7308f1fc6fa3bcd7d373bb2588c3418ff6742f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clear_canvas.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 538
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::clear_canvas::clear_canvas
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":13248,"rva":10855904,"va":5379565024}
// 编译实例 RVAs: 0xa5a5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5078974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8be7f4d00","size":5,"soft_fail":false,"target_va":5384644912,"va":5379565933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5043480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e818f54c00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379565923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-3887896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8e8acc4ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379566595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f890
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10337637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e89b4262ff","size":5,"soft_fail":false,"target_va":5369231504,"va":5379569136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10267925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8eb5263ff","size":5,"soft_fail":false,"target_va":5369301120,"va":5379569040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400963b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10244776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e858ad63ff","size":5,"soft_fail":false,"target_va":5369324464,"va":5379569235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c2ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10059581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8c38066ff","size":5,"soft_fail":false,"target_va":5369506720,"va":5379566296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10049722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e846a766ff","size":5,"soft_fail":false,"target_va":5369517680,"va":5379567397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=6
//     target_aliases=34 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9906882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e83ed568ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5379567245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-956664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e80867f1ff","size":5,"soft_fail":false,"target_va":5378612848,"va":5379569507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140994960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8a499f3ff","size":5,"soft_fail":false,"target_va":5378754912,"va":5379567543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-750919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8b98af4ff","size":5,"soft_fail":false,"target_va":5378816768,"va":5379567682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ab0970
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t351041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8415b0500","size":5,"soft_fail":false,"target_va":5379918192,"va":5379567146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b45c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t961910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e876ad0e00","size":5,"soft_fail":false,"target_va":5380529152,"va":5379567237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2010747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e87bae1e00","size":5,"soft_fail":false,"target_va":5381577744,"va":5379566992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88a40
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2284989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8bddd2200","size":5,"soft_fail":false,"target_va":5381851712,"va":5379566718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca8000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2411695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8afcc2400","size":5,"soft_fail":false,"target_va":5381980160,"va":5379568460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37490705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e811103c02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379566346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37490621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8bd0f3c02","size":5,"soft_fail":false,"target_va":5417057088,"va":5379566462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37052635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8db603502","size":5,"soft_fail":false,"target_va":5416621600,"va":5379568960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t430697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e869920600","size":5,"soft_fail":false,"target_va":5379997744,"va":5379567042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1734409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e809771a00","size":5,"soft_fail":false,"target_va":5381301072,"va":5379566658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t5157845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e8d5b34e00","size":5,"soft_fail":false,"target_va":5384726688,"va":5379568838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37050407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379565024,"raw_hex":"e827583502","size":5,"soft_fail":false,"target_va":5416619360,"va":5379568948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379565024,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379567013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":18648,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":538,"record_offset":17964,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":538,"record_offset":14572,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":538,"record_offset":13476,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":14936,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":23176,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":27944,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::GateColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":23724,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":33000,"source_line":182,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::HalftimberColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":24728,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":33002,"source_line":306,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::StoneFoundationColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":26736,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":33006,"source_line":362,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":22720,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32997,"source_line":123,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WoodenWallColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":25732,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":33004,"source_line":332,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":538,"record_offset":21948,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":29512,"source_line":991,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":538,"record_offset":19920,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32987,"source_line":88,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging_ssbo
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":538,"record_offset":20804,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":29020,"source_line":307,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":538,"record_offset":27800,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":33007,"source_line":120,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/curved_halftimber.rs
//     evidence={"module":538,"record_offset":21256,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32424,"source_line":14,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":538,"record_offset":15104,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":17254,"source_line":44,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::reset_max_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":538,"record_offset":19668,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32986,"source_line":115,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_render_slots::WallRenderSlots::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_render_slots.rs
//     evidence={"module":538,"record_offset":22184,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32995,"source_line":116,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":538,"record_offset":21400,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":32992,"source_line":109,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":538,"record_offset":14992,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":14852,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":18144,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":13248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":13248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":13248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":19112,"parent_record_offset":19012,"ancestor_record_offset":13248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":13248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":19044,"parent_record_offset":19012,"ancestor_record_offset":13248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":19012,"parent_record_offset":18984,"ancestor_record_offset":13248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":13248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":13248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":18984,"parent_record_offset":18956,"ancestor_record_offset":13248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":13248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":18680,"parent_record_offset":18648,"ancestor_record_offset":13248,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":13828,"parent_record_offset":13800,"ancestor_record_offset":13248,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":13800,"parent_record_offset":13772,"ancestor_record_offset":13248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":22660,"parent_record_offset":22184,"ancestor_record_offset":13248,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":21480,"parent_record_offset":21400,"ancestor_record_offset":13248,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":13772,"parent_record_offset":13636,"ancestor_record_offset":13248,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":538,"record_offset":17992,"parent_record_offset":17964,"ancestor_record_offset":13248,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":538,"record_offset":13636,"parent_record_offset":13588,"ancestor_record_offset":13248,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":538,"record_offset":14616,"parent_record_offset":14572,"ancestor_record_offset":13248,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":538,"record_offset":13588,"parent_record_offset":13524,"ancestor_record_offset":13248,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":538,"record_offset":13524,"parent_record_offset":13476,"ancestor_record_offset":13248,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":538,"record_offset":15708,"parent_record_offset":15680,"ancestor_record_offset":13248,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":538,"record_offset":15608,"parent_record_offset":15404,"ancestor_record_offset":13248,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":538,"record_offset":23000,"parent_record_offset":22944,"ancestor_record_offset":13248,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":538,"record_offset":14664,"parent_record_offset":14616,"ancestor_record_offset":13248,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":538,"record_offset":19988,"parent_record_offset":19964,"ancestor_record_offset":13248,"inlinee":7947,"source_line":99,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":538,"record_offset":19964,"parent_record_offset":19920,"ancestor_record_offset":13248,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":538,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":13248,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":538,"record_offset":20016,"parent_record_offset":19988,"ancestor_record_offset":13248,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":538,"record_offset":15572,"parent_record_offset":15548,"ancestor_record_offset":13248,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":23468,"parent_record_offset":23440,"ancestor_record_offset":13248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":14220,"parent_record_offset":14188,"ancestor_record_offset":13248,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":23496,"parent_record_offset":23468,"ancestor_record_offset":13248,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":13716,"parent_record_offset":13636,"ancestor_record_offset":13248,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":19080,"parent_record_offset":19044,"ancestor_record_offset":13248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":538,"record_offset":15736,"parent_record_offset":15680,"ancestor_record_offset":13248,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":538,"record_offset":15316,"parent_record_offset":15284,"ancestor_record_offset":13248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":538,"record_offset":20100,"parent_record_offset":20016,"ancestor_record_offset":13248,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":14712,"parent_record_offset":14664,"ancestor_record_offset":13248,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":13248,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":14784,"parent_record_offset":14664,"ancestor_record_offset":13248,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":20904,"parent_record_offset":20804,"ancestor_record_offset":13248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":15436,"parent_record_offset":15404,"ancestor_record_offset":13248,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":14816,"parent_record_offset":14616,"ancestor_record_offset":13248,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":538,"record_offset":21680,"parent_record_offset":21652,"ancestor_record_offset":13248,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":18220,"parent_record_offset":18192,"ancestor_record_offset":13248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13248,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":15524,"parent_record_offset":15404,"ancestor_record_offset":13248,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":23532,"parent_record_offset":23408,"ancestor_record_offset":13248,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":13248,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":28024,"parent_record_offset":27944,"ancestor_record_offset":13248,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":538,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":13248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":13248,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":13248,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":538,"record_offset":20428,"parent_record_offset":19920,"ancestor_record_offset":13248,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":538,"record_offset":14124,"parent_record_offset":13636,"ancestor_record_offset":13248,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":538,"record_offset":14412,"parent_record_offset":13636,"ancestor_record_offset":13248,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":538,"record_offset":20652,"parent_record_offset":20568,"ancestor_record_offset":13248,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":538,"record_offset":14188,"parent_record_offset":14156,"ancestor_record_offset":13248,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":538,"record_offset":14156,"parent_record_offset":14124,"ancestor_record_offset":13248,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":538,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":13248,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":538,"record_offset":20676,"parent_record_offset":20652,"ancestor_record_offset":13248,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":20172,"parent_record_offset":20148,"ancestor_record_offset":13248,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":20044,"parent_record_offset":20016,"ancestor_record_offset":13248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":538,"record_offset":20480,"parent_record_offset":20452,"ancestor_record_offset":13248,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":538,"record_offset":20452,"parent_record_offset":20428,"ancestor_record_offset":13248,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":13248,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14900,"parent_record_offset":14876,"ancestor_record_offset":13248,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":538,"record_offset":20844,"parent_record_offset":20804,"ancestor_record_offset":13248,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":538,"record_offset":20868,"parent_record_offset":20844,"ancestor_record_offset":13248,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":538,"record_offset":20940,"parent_record_offset":20804,"ancestor_record_offset":13248,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::WallColorTable::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":538,"record_offset":22744,"parent_record_offset":22720,"ancestor_record_offset":13248,"inlinee":32998,"source_line":60,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":538,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":13248,"inlinee":29514,"source_line":45,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::plant_sample_raster::PlantSampleRaster::mark_all_dirty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":538,"record_offset":20396,"parent_record_offset":19920,"ancestor_record_offset":13248,"inlinee":32990,"source_line":327,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::plant_sample_raster::impl$0::clear::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":538,"record_offset":20148,"parent_record_offset":20124,"ancestor_record_offset":13248,"inlinee":32989,"source_line":93,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":538,"record_offset":21452,"parent_record_offset":21400,"ancestor_record_offset":13248,"inlinee":32994,"source_line":64,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":538,"record_offset":18440,"parent_record_offset":18380,"ancestor_record_offset":13248,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":538,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":13248,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":13248,"inlinee":12661,"source_line":872,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":13248,"inlinee":12651,"source_line":866,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":23408,"parent_record_offset":23376,"ancestor_record_offset":13248,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18316,"parent_record_offset":18292,"ancestor_record_offset":13248,"inlinee":5023,"source_line":3207,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18380,"parent_record_offset":18316,"ancestor_record_offset":13248,"inlinee":5024,"source_line":2669,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":13248,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":23376,"parent_record_offset":23320,"ancestor_record_offset":13248,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18352,"parent_record_offset":18316,"ancestor_record_offset":13248,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18412,"parent_record_offset":18380,"ancestor_record_offset":13248,"inlinee":5025,"source_line":2691,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":23440,"parent_record_offset":23408,"ancestor_record_offset":13248,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18528,"parent_record_offset":18316,"ancestor_record_offset":13248,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":13248,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":538,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":13248,"inlinee":12650,"source_line":878,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\scopeguard.rs
//     evidence={"module":538,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":13248,"inlinee":12554,"source_line":69,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":15040,"parent_record_offset":15016,"ancestor_record_offset":13248,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":538,"record_offset":15016,"parent_record_offset":14992,"ancestor_record_offset":13248,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":13248,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":21284,"parent_record_offset":21256,"ancestor_record_offset":13248,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":21508,"parent_record_offset":21400,"ancestor_record_offset":13248,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":15848,"parent_record_offset":15172,"ancestor_record_offset":13248,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":15172,"parent_record_offset":15104,"ancestor_record_offset":13248,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":538,"record_offset":15880,"parent_record_offset":15104,"ancestor_record_offset":13248,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":538,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":13248,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":538,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":13248,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":538,"record_offset":15344,"parent_record_offset":15316,"ancestor_record_offset":13248,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":538,"record_offset":15680,"parent_record_offset":15284,"ancestor_record_offset":13248,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":538,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":13248,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":538,"record_offset":15372,"parent_record_offset":15344,"ancestor_record_offset":13248,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":538,"record_offset":15404,"parent_record_offset":15372,"ancestor_record_offset":13248,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":538,"record_offset":15284,"parent_record_offset":15256,"ancestor_record_offset":13248,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":538,"record_offset":15256,"parent_record_offset":15228,"ancestor_record_offset":13248,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00538.jsonl.gz"}
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
// 完整结构及原始类型记录: clear_canvas.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clear_canvas.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clear_canvas.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>

// ------------------------------------------------------------------
// 函数: country_core::systems::clear_canvas::clear_canvas
// 信息 ID: 4f0a45c7bf591de1e27e3403
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8fc1a7b30f2bcee7e3c5e6ad: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)> @ 0x7cdbf -> 0xa5a5e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)> > @ 0x2b8a72 -> 0xa5a5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
