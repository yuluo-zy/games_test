// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/history/replay_wall.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a2b060723ddf8cafc49c6fb225882d458b0cb159747196a8a2dd70180c2bfed1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 416, 474, 484, 1308, 1321
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":416,"record_offset":22156,"rva":5790432,"va":5374499552}
// 编译实例 RVAs: 0x585ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1749973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e82b4ce5ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374499664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1750295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e8e94ae5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374500002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406248b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t650361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e879ec0900","size":5,"soft_fail":false,"target_va":5375150256,"va":5374499890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9466651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e81b739000","size":5,"soft_fail":false,"target_va":5383966368,"va":5374499712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29093603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e8e3eebb01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374499752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28478457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e8f98bb201","size":5,"soft_fail":false,"target_va":5402978144,"va":5374499682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3027202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374499552,"raw_hex":"e802312e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5374499913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":416,"record_offset":23616,"parent_record_offset":22156,"ancestor_record_offset":22156,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":416,"record_offset":22312,"parent_record_offset":22156,"ancestor_record_offset":22156,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":22696,"parent_record_offset":22672,"ancestor_record_offset":22156,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":22720,"parent_record_offset":22696,"ancestor_record_offset":22156,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":23568,"parent_record_offset":23544,"ancestor_record_offset":22156,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":22672,"parent_record_offset":22572,"ancestor_record_offset":22156,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":23544,"parent_record_offset":23452,"ancestor_record_offset":22156,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":23480,"parent_record_offset":23452,"ancestor_record_offset":22156,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":22572,"parent_record_offset":22540,"ancestor_record_offset":22156,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":22540,"parent_record_offset":22508,"ancestor_record_offset":22156,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":22788,"parent_record_offset":22540,"ancestor_record_offset":22156,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":23452,"parent_record_offset":23428,"ancestor_record_offset":22156,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":22468,"parent_record_offset":22432,"ancestor_record_offset":22156,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":22432,"parent_record_offset":22396,"ancestor_record_offset":22156,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":416,"record_offset":22396,"parent_record_offset":22360,"ancestor_record_offset":22156,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":22508,"parent_record_offset":22468,"ancestor_record_offset":22156,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":22360,"parent_record_offset":22312,"ancestor_record_offset":22156,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":416,"record_offset":22616,"parent_record_offset":22572,"ancestor_record_offset":22156,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":416,"record_offset":22640,"parent_record_offset":22616,"ancestor_record_offset":22156,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":416,"record_offset":22856,"parent_record_offset":22832,"ancestor_record_offset":22156,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":416,"record_offset":23508,"parent_record_offset":23480,"ancestor_record_offset":22156,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":416,"record_offset":22832,"parent_record_offset":22468,"ancestor_record_offset":22156,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":23404,"parent_record_offset":22312,"ancestor_record_offset":22156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":22744,"parent_record_offset":22720,"ancestor_record_offset":22156,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":416,"record_offset":18700,"rva":5789040,"va":5374498160}
// 编译实例 RVAs: 0x585570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1749580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e8b44de5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374499287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140664ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t914545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e871f40d00","size":5,"soft_fail":false,"target_va":5375413152,"va":5374498602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1384561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e871201500","size":5,"soft_fail":false,"target_va":5375883328,"va":5374498762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29094136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e8f8f0bb01","size":5,"soft_fail":false,"target_va":5403593376,"va":5374499235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3193833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e81744cfff","size":5,"soft_fail":false,"target_va":5371305056,"va":5374498884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28479097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e8798eb201","size":5,"soft_fail":false,"target_va":5402978432,"va":5374499330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e889e3ffff","size":5,"soft_fail":false,"target_va":5374491968,"va":5374499250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2854915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374498160,"raw_hex":"e803902b00","size":5,"soft_fail":false,"target_va":5377353776,"va":5374498856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19640,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":20208,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":19228,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":19340,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":21108,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":19872,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":20972,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":416,"record_offset":18912,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":416,"record_offset":20600,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":416,"record_offset":20628,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":416,"record_offset":19464,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":20488,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":19256,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":20684,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":416,"record_offset":19984,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":416,"record_offset":18976,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::is_done
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":416,"record_offset":20180,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":23768,"source_line":747,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":416,"record_offset":20656,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":416,"record_offset":20516,"parent_record_offset":18700,"ancestor_record_offset":18700,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":18700,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":21560,"parent_record_offset":21468,"ancestor_record_offset":18700,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":20384,"parent_record_offset":20208,"ancestor_record_offset":18700,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":18700,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19024,"parent_record_offset":19000,"ancestor_record_offset":18700,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":20248,"parent_record_offset":20208,"ancestor_record_offset":18700,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19152,"parent_record_offset":19024,"ancestor_record_offset":18700,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19056,"parent_record_offset":19024,"ancestor_record_offset":18700,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19084,"parent_record_offset":19056,"ancestor_record_offset":18700,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":416,"record_offset":19112,"parent_record_offset":19084,"ancestor_record_offset":18700,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":21496,"parent_record_offset":21468,"ancestor_record_offset":18700,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":19184,"parent_record_offset":19152,"ancestor_record_offset":18700,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":21468,"parent_record_offset":21444,"ancestor_record_offset":18700,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":19376,"parent_record_offset":19340,"ancestor_record_offset":18700,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":21156,"parent_record_offset":21108,"ancestor_record_offset":18700,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":21316,"parent_record_offset":21108,"ancestor_record_offset":18700,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":18700,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":416,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":18700,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":416,"record_offset":18940,"parent_record_offset":18912,"ancestor_record_offset":18700,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":20756,"parent_record_offset":20712,"ancestor_record_offset":18700,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":18700,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":416,"record_offset":21524,"parent_record_offset":21496,"ancestor_record_offset":18700,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":416,"record_offset":20332,"parent_record_offset":20304,"ancestor_record_offset":18700,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":21396,"parent_record_offset":21372,"ancestor_record_offset":18700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":416,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":18700,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":19600,"parent_record_offset":19576,"ancestor_record_offset":18700,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":21048,"parent_record_offset":20972,"ancestor_record_offset":18700,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":19544,"parent_record_offset":19500,"ancestor_record_offset":18700,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":20828,"parent_record_offset":20784,"ancestor_record_offset":18700,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":19432,"parent_record_offset":19340,"ancestor_record_offset":18700,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":19896,"parent_record_offset":19872,"ancestor_record_offset":18700,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":416,"record_offset":20712,"parent_record_offset":20684,"ancestor_record_offset":18700,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":416,"record_offset":19280,"parent_record_offset":19256,"ancestor_record_offset":18700,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":416,"record_offset":19576,"parent_record_offset":19500,"ancestor_record_offset":18700,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":416,"record_offset":19500,"parent_record_offset":19464,"ancestor_record_offset":18700,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":20540,"parent_record_offset":20516,"ancestor_record_offset":18700,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":18700,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6934
// 首条位置: {"module":474,"inlinee":26932}
// 编译实例 RVAs: 0x6648c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4107831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375412416,"raw_hex":"e8c951c1ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5375413058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1526969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375412416,"raw_hex":"e8b94c1700","size":5,"soft_fail":false,"target_va":5376939952,"va":5375412978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":60520,"parent_record_offset":58860,"ancestor_record_offset":58860,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":59016,"parent_record_offset":58860,"ancestor_record_offset":58860,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":60712,"parent_record_offset":60680,"ancestor_record_offset":58860,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":60680,"parent_record_offset":60640,"ancestor_record_offset":58860,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":60640,"parent_record_offset":60604,"ancestor_record_offset":58860,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":60572,"parent_record_offset":60544,"ancestor_record_offset":58860,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":60544,"parent_record_offset":60520,"ancestor_record_offset":58860,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":60604,"parent_record_offset":60520,"ancestor_record_offset":58860,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":474,"record_offset":59468,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":59320,"parent_record_offset":59292,"ancestor_record_offset":58860,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":59348,"parent_record_offset":59292,"ancestor_record_offset":58860,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":59408,"parent_record_offset":59384,"ancestor_record_offset":58860,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":59236,"parent_record_offset":59212,"ancestor_record_offset":58860,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":59180,"parent_record_offset":59156,"ancestor_record_offset":58860,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":59644,"parent_record_offset":59620,"ancestor_record_offset":58860,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":60744,"parent_record_offset":60640,"ancestor_record_offset":58860,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":59588,"parent_record_offset":59552,"ancestor_record_offset":58860,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":59044,"parent_record_offset":59016,"ancestor_record_offset":58860,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59440,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59268,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59292,"parent_record_offset":59268,"ancestor_record_offset":58860,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59212,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":60420,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59156,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":59384,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":59524,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":59492,"parent_record_offset":59468,"ancestor_record_offset":58860,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":474,"record_offset":59620,"parent_record_offset":59588,"ancestor_record_offset":58860,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":59552,"parent_record_offset":59072,"ancestor_record_offset":58860,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":59072,"parent_record_offset":59044,"ancestor_record_offset":58860,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5D02
// 首条位置: {"module":484,"inlinee":23810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":484,"record_offset":16812,"parent_record_offset":16752,"ancestor_record_offset":16752,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":17032,"parent_record_offset":16752,"ancestor_record_offset":16752,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":16948,"parent_record_offset":16752,"ancestor_record_offset":16752,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":16840,"parent_record_offset":16812,"ancestor_record_offset":16752,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":484,"record_offset":17056,"parent_record_offset":17032,"ancestor_record_offset":16752,"inlinee":23770,"source_line":2786,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":484,"record_offset":17164,"parent_record_offset":17132,"ancestor_record_offset":16752,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":484,"record_offset":17108,"parent_record_offset":17056,"ancestor_record_offset":16752,"inlinee":23772,"source_line":12,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":16752,"inlinee":23722,"source_line":900,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":16752,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":484,"record_offset":17080,"parent_record_offset":17056,"ancestor_record_offset":16752,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":484,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":16752,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":16752,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":16752,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6930
// 首条位置: {"module":474,"inlinee":26928}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1308,"record_offset":222740,"parent_record_offset":222632,"ancestor_record_offset":222632,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1308,"record_offset":222656,"parent_record_offset":222632,"ancestor_record_offset":222632,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":222764,"parent_record_offset":222740,"ancestor_record_offset":222632,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1308,"record_offset":222788,"parent_record_offset":222764,"ancestor_record_offset":222632,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1308,"record_offset":222704,"parent_record_offset":222680,"ancestor_record_offset":222632,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1308,"record_offset":222680,"parent_record_offset":222656,"ancestor_record_offset":222632,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF136
// 首条位置: {"module":1321,"inlinee":61750}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1321,"record_offset":11516,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1321,"record_offset":11660,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1321,"record_offset":11632,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1321,"record_offset":11604,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1321,"record_offset":11544,"parent_record_offset":11516,"ancestor_record_offset":11464,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1321,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":11464,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":474,"record_offset":60788,"rva":6704032,"va":5375413152}
// 编译实例 RVAs: 0x664ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2670042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e82642d7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375419749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406248b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-265382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e85af3fbff","size":5,"soft_fail":false,"target_va":5375150256,"va":5375415633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140624b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-267173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e85becfbff","size":5,"soft_fail":false,"target_va":5375150848,"va":5375418016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140646160
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-128019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8ed0bfeff","size":5,"soft_fail":false,"target_va":5375287648,"va":5375415662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065dc30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e84a8dffff","size":5,"soft_fail":false,"target_va":5375384624,"va":5375413985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=4
//     target_aliases=14 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t465107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8d3180700","size":5,"soft_fail":false,"target_va":5375881440,"va":5375416328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1529876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e814581700","size":5,"soft_fail":false,"target_va":5376943968,"va":5375414087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4237682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e872a94000","size":5,"soft_fail":false,"target_va":5379651280,"va":5375413593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d732b0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7397192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e848df7000","size":5,"soft_fail":false,"target_va":5382812336,"va":5375415139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d73340
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7397320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8c8df7000","size":5,"soft_fail":false,"target_va":5382812480,"va":5375415155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e87b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8526642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8321b8200","size":5,"soft_fail":false,"target_va":5383945008,"va":5375418361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8550173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e81d778200","size":5,"soft_fail":false,"target_va":5383966736,"va":5375416558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f373c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9250448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e890268d00","size":5,"soft_fail":false,"target_va":5384664000,"va":5375413547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4106023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8d958c1ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375413682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventWriter$LT$E$GT$::send
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3429017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e899523400","size":5,"soft_fail":false,"target_va":5378842928,"va":5375413906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27559591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8a786a401","size":5,"soft_fail":false,"target_va":5402979152,"va":5375419556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27559505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e85186a401","size":5,"soft_fail":false,"target_va":5402978944,"va":5375419434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::panic_deprecated_operation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-599469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e853daf6ff","size":5,"soft_fail":false,"target_va":5374820000,"va":5375419464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6451722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e80a726200","size":5,"soft_fail":false,"target_va":5381870304,"va":5375418577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-133245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e883f7fdff","size":5,"soft_fail":false,"target_va":5375285728,"va":5375418968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-129338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8c606feff","size":5,"soft_fail":false,"target_va":5375286960,"va":5375416293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-128509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8030afeff","size":5,"soft_fail":false,"target_va":5375287584,"va":5375416088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-128808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8d808feff","size":5,"soft_fail":false,"target_va":5375287616,"va":5375416419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-128840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8b808feff","size":5,"soft_fail":false,"target_va":5375287552,"va":5375416387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-129829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8db04feff","size":5,"soft_fail":false,"target_va":5375287696,"va":5375417520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::_deprecated_morph_curve
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-913879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8290ef2ff","size":5,"soft_fail":false,"target_va":5374499552,"va":5375413426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e829efffff","size":5,"soft_fail":false,"target_va":5375412416,"va":5375416722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t306325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e895ac0400","size":5,"soft_fail":false,"target_va":5375723824,"va":5375417494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1938679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8f7941d00","size":5,"soft_fail":false,"target_va":5377353776,"va":5375415092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1521213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e83d361700","size":5,"soft_fail":false,"target_va":5376938064,"va":5375416846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1525458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8d2461700","size":5,"soft_fail":false,"target_va":5376938816,"va":5375413353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2553737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e889f72600","size":5,"soft_fail":false,"target_va":5377969360,"va":5375415618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2554867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8f3fb2600","size":5,"soft_fail":false,"target_va":5377970176,"va":5375415304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2538456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e8d8bb2600","size":5,"soft_fail":false,"target_va":5377954064,"va":5375415603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2551668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e874ef2600","size":5,"soft_fail":false,"target_va":5377966512,"va":5375414839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1525306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375413152,"raw_hex":"e83a461700","size":5,"soft_fail":false,"target_va":5376939952,"va":5375414641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":61024,"parent_record_offset":60788,"ancestor_record_offset":60788,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":61056,"parent_record_offset":60788,"ancestor_record_offset":60788,"inlinee":26929,"source_line":506,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":66220,"parent_record_offset":60788,"ancestor_record_offset":60788,"inlinee":26930,"source_line":258,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":64712,"parent_record_offset":64688,"ancestor_record_offset":60788,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":64688,"parent_record_offset":64588,"ancestor_record_offset":60788,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":64620,"parent_record_offset":64588,"ancestor_record_offset":60788,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":61924,"parent_record_offset":61892,"ancestor_record_offset":60788,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":64588,"parent_record_offset":64564,"ancestor_record_offset":60788,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":61892,"parent_record_offset":61848,"ancestor_record_offset":60788,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":61848,"parent_record_offset":61812,"ancestor_record_offset":60788,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":61780,"parent_record_offset":61752,"ancestor_record_offset":60788,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":61752,"parent_record_offset":61728,"ancestor_record_offset":60788,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":61728,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":61812,"parent_record_offset":61728,"ancestor_record_offset":60788,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":474,"record_offset":65048,"parent_record_offset":64816,"ancestor_record_offset":60788,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":474,"record_offset":62884,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":62736,"parent_record_offset":62708,"ancestor_record_offset":60788,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":62764,"parent_record_offset":62708,"ancestor_record_offset":60788,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":474,"record_offset":66160,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":474,"record_offset":65076,"parent_record_offset":64816,"ancestor_record_offset":60788,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":474,"record_offset":64652,"parent_record_offset":64620,"ancestor_record_offset":60788,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62048,"parent_record_offset":61996,"ancestor_record_offset":60788,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62104,"parent_record_offset":61996,"ancestor_record_offset":60788,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62596,"parent_record_offset":62572,"ancestor_record_offset":60788,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":61616,"parent_record_offset":61592,"ancestor_record_offset":60788,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":65564,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":68224,"parent_record_offset":66220,"ancestor_record_offset":60788,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":66580,"parent_record_offset":66220,"ancestor_record_offset":60788,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":64540,"parent_record_offset":64500,"ancestor_record_offset":60788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":61956,"parent_record_offset":61848,"ancestor_record_offset":60788,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":474,"record_offset":65536,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":474,"record_offset":64988,"parent_record_offset":64964,"ancestor_record_offset":60788,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":474,"record_offset":65012,"parent_record_offset":64988,"ancestor_record_offset":60788,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":62424,"parent_record_offset":62388,"ancestor_record_offset":60788,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":63008,"parent_record_offset":62968,"ancestor_record_offset":60788,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":62456,"parent_record_offset":62424,"ancestor_record_offset":60788,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":68280,"parent_record_offset":68256,"ancestor_record_offset":60788,"inlinee":22425,"source_line":172,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":64452,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_try_update_state
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61440,"parent_record_offset":61412,"ancestor_record_offset":60788,"inlinee":24555,"source_line":185,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64208,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":26490,"source_line":137,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61412,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":26489,"source_line":131,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":66132,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":23484,"source_line":125,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":65172,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64792,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":24554,"source_line":173,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_arch_height::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64352,"parent_record_offset":64236,"ancestor_record_offset":60788,"inlinee":26171,"source_line":138,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_elevation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61556,"parent_record_offset":61440,"ancestor_record_offset":60788,"inlinee":26185,"source_line":132,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_wall_state_enclosure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64936,"parent_record_offset":64816,"ancestor_record_offset":60788,"inlinee":24557,"source_line":174,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":68348,"parent_record_offset":68316,"ancestor_record_offset":60788,"inlinee":26935,"source_line":303,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":68256,"parent_record_offset":68224,"ancestor_record_offset":60788,"inlinee":26934,"source_line":401,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":62388,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":26932,"source_line":69,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":65140,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":23580,"source_line":101,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":62360,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":23590,"source_line":79,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":64760,"parent_record_offset":64452,"ancestor_record_offset":60788,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62856,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62684,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62708,"parent_record_offset":62684,"ancestor_record_offset":60788,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62628,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":63840,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62572,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62800,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":62940,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":62908,"parent_record_offset":62884,"ancestor_record_offset":60788,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":474,"record_offset":61524,"parent_record_offset":61500,"ancestor_record_offset":60788,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":474,"record_offset":61592,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":474,"record_offset":65104,"parent_record_offset":64816,"ancestor_record_offset":60788,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":474,"record_offset":64964,"parent_record_offset":64816,"ancestor_record_offset":60788,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":474,"record_offset":61472,"parent_record_offset":61440,"ancestor_record_offset":60788,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":474,"record_offset":61500,"parent_record_offset":61440,"ancestor_record_offset":60788,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":474,"record_offset":62136,"parent_record_offset":61056,"ancestor_record_offset":60788,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":474,"record_offset":64500,"parent_record_offset":64452,"ancestor_record_offset":60788,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":62968,"parent_record_offset":62488,"ancestor_record_offset":60788,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":62488,"parent_record_offset":62456,"ancestor_record_offset":60788,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6932
// 首条位置: {"module":474,"inlinee":26930}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":68224,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":66580,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":67316,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":66968,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":68164,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":23484,"source_line":125,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":68108,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":68192,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":26932,"source_line":69,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":66516,"parent_record_offset":66220,"ancestor_record_offset":66220,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":66884,"parent_record_offset":66860,"ancestor_record_offset":66220,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":66860,"parent_record_offset":66760,"ancestor_record_offset":66220,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":66792,"parent_record_offset":66760,"ancestor_record_offset":66220,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":66760,"parent_record_offset":66736,"ancestor_record_offset":66220,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":474,"record_offset":66824,"parent_record_offset":66792,"ancestor_record_offset":66220,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":66712,"parent_record_offset":66672,"ancestor_record_offset":66220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":68280,"parent_record_offset":68256,"ancestor_record_offset":66220,"inlinee":22425,"source_line":172,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":66620,"parent_record_offset":66580,"ancestor_record_offset":66220,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":68348,"parent_record_offset":68316,"ancestor_record_offset":66220,"inlinee":26935,"source_line":303,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":68256,"parent_record_offset":68224,"ancestor_record_offset":66220,"inlinee":26934,"source_line":401,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":66932,"parent_record_offset":66620,"ancestor_record_offset":66220,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":474,"record_offset":66672,"parent_record_offset":66620,"ancestor_record_offset":66220,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 303
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6937
// 首条位置: {"module":474,"inlinee":26935}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":68372,"parent_record_offset":68348,"ancestor_record_offset":68348,"inlinee":22425,"source_line":172,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6936
// 首条位置: {"module":474,"inlinee":26934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":68280,"parent_record_offset":68256,"ancestor_record_offset":68256,"inlinee":22425,"source_line":172,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 506
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6931
// 首条位置: {"module":474,"inlinee":26929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":61728,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":474,"record_offset":66160,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":474,"record_offset":65564,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":474,"record_offset":65536,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":474,"record_offset":64452,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64208,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":26490,"source_line":137,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61412,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":26489,"source_line":131,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":66132,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":23484,"source_line":125,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":65172,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64792,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":24554,"source_line":173,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":474,"record_offset":62388,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":26932,"source_line":69,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":65140,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":23580,"source_line":101,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":62360,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":23590,"source_line":79,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":474,"record_offset":61592,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":474,"record_offset":62136,"parent_record_offset":61056,"ancestor_record_offset":61056,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":64712,"parent_record_offset":64688,"ancestor_record_offset":61056,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":64688,"parent_record_offset":64588,"ancestor_record_offset":61056,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":64620,"parent_record_offset":64588,"ancestor_record_offset":61056,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":61924,"parent_record_offset":61892,"ancestor_record_offset":61056,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":474,"record_offset":64588,"parent_record_offset":64564,"ancestor_record_offset":61056,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":61892,"parent_record_offset":61848,"ancestor_record_offset":61056,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":61848,"parent_record_offset":61812,"ancestor_record_offset":61056,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":61780,"parent_record_offset":61752,"ancestor_record_offset":61056,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":474,"record_offset":61752,"parent_record_offset":61728,"ancestor_record_offset":61056,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":63940,"parent_record_offset":62388,"ancestor_record_offset":61056,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":474,"record_offset":61812,"parent_record_offset":61728,"ancestor_record_offset":61056,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":474,"record_offset":65048,"parent_record_offset":64816,"ancestor_record_offset":61056,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":474,"record_offset":62884,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":62736,"parent_record_offset":62708,"ancestor_record_offset":61056,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":474,"record_offset":62764,"parent_record_offset":62708,"ancestor_record_offset":61056,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":474,"record_offset":65076,"parent_record_offset":64816,"ancestor_record_offset":61056,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":474,"record_offset":64652,"parent_record_offset":64620,"ancestor_record_offset":61056,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62048,"parent_record_offset":61996,"ancestor_record_offset":61056,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62104,"parent_record_offset":61996,"ancestor_record_offset":61056,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":62596,"parent_record_offset":62572,"ancestor_record_offset":61056,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":474,"record_offset":61616,"parent_record_offset":61592,"ancestor_record_offset":61056,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":64540,"parent_record_offset":64500,"ancestor_record_offset":61056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":61956,"parent_record_offset":61848,"ancestor_record_offset":61056,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":474,"record_offset":64988,"parent_record_offset":64964,"ancestor_record_offset":61056,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":474,"record_offset":65012,"parent_record_offset":64988,"ancestor_record_offset":61056,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":62424,"parent_record_offset":62388,"ancestor_record_offset":61056,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":63008,"parent_record_offset":62968,"ancestor_record_offset":61056,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":474,"record_offset":62456,"parent_record_offset":62424,"ancestor_record_offset":61056,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_try_update_state
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61440,"parent_record_offset":61412,"ancestor_record_offset":61056,"inlinee":24555,"source_line":185,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_arch_height::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64352,"parent_record_offset":64236,"ancestor_record_offset":61056,"inlinee":26171,"source_line":138,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_elevation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":61556,"parent_record_offset":61440,"ancestor_record_offset":61056,"inlinee":26185,"source_line":132,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_wall_state_enclosure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":474,"record_offset":64936,"parent_record_offset":64816,"ancestor_record_offset":61056,"inlinee":24557,"source_line":174,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":474,"record_offset":64760,"parent_record_offset":64452,"ancestor_record_offset":61056,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62856,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62684,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62708,"parent_record_offset":62684,"ancestor_record_offset":61056,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62628,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":63840,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62572,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":474,"record_offset":62800,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":62940,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":474,"record_offset":62908,"parent_record_offset":62884,"ancestor_record_offset":61056,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":474,"record_offset":61524,"parent_record_offset":61500,"ancestor_record_offset":61056,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":474,"record_offset":63040,"parent_record_offset":63008,"ancestor_record_offset":61056,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":474,"record_offset":65104,"parent_record_offset":64816,"ancestor_record_offset":61056,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":474,"record_offset":64964,"parent_record_offset":64816,"ancestor_record_offset":61056,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":474,"record_offset":61472,"parent_record_offset":61440,"ancestor_record_offset":61056,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":474,"record_offset":61500,"parent_record_offset":61440,"ancestor_record_offset":61056,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":474,"record_offset":64500,"parent_record_offset":64452,"ancestor_record_offset":61056,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":62968,"parent_record_offset":62488,"ancestor_record_offset":61056,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":474,"record_offset":62488,"parent_record_offset":62456,"ancestor_record_offset":61056,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::is_done
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 747
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CD8
// 首条位置: {"module":416,"inlinee":23768}
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
// 完整结构及原始类型记录: replay_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::history::replay_wall::WallReplayCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayEditCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 17
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>", "constructor": "country_core::systems::history::replay_wall::WallReplayEditCmd::process", "arguments": ["country_core::systems::history::replay_wall::replay_wall::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::history::replay_wall::WallReplayEditCmd,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayCmd::new
// 信息 ID: c8c7a832320bd6be2b78d306
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=16752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::is_done
// 信息 ID: bdd44d63b2d42ed126c03edc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::replay_wall; module=416, inline_record=20180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>
// 信息 ID: 66eb1ae09a5d5d041060c5da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::history::replay_wall::WallReplayEditCmd::process -> ["country_core::systems::history::replay_wall::replay_wall::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::replay_wall @ 0x58572a -> 0x664ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 信息 ID: 5f9644aec7db7d8352d68d9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=61056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 信息 ID: c49ca2dbdc9172a6b4493cf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=66220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 信息 ID: f05014c23160b37c2d1ee975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0x664cb2 -> 0x585ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0x6656a7 -> 0x585ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 信息 ID: 76ee43e7d68caf62932ed313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; module=1308, inline_record=222632; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; module=1308, inline_record=223968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 信息 ID: 6bb74b13149e52908b0a56db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::history::replay_wall::WallReplayEditCmd,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; module=1321, inline_record=11464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 信息 ID: ac1b3e17f0a8ea07f65436ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=68348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 信息 ID: 8755df029b6bc31fedce2b24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=68256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall
// 信息 ID: 87714b1238241cd221ce6bd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] db75246fbca052d72f1962b8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> @ 0x663d1f -> 0x585570 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> > @ 0xe62e9e -> 0x585570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall::closure$0
// 信息 ID: aedb9574bd2a1d1a88f31c9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::replay_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::replay_wall::closure_env$0 <- country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=62388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=68192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
