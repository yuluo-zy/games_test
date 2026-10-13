// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/ui_systems/move_ops.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ecd839a88eb7786b7c9ea3c62c863d8d3dc6aae16591968c2b37f5093ffe39e2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: move_ops.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4376
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4376,"record_offset":1128,"rva":31875088,"va":5400584208}
// 编译实例 RVAs: 0x1e66010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27834807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e8494657fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400584498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27834927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e8d14557fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400584634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2393474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e882852400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400584665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23054571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e81537a0fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5400584454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e8dd097ffe","size":5,"soft_fail":false,"target_va":5375355744,"va":5400584574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18310983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e8b998e8fe","size":5,"soft_fail":false,"target_va":5382273632,"va":5400584610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23640985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584208,"raw_hex":"e8674497fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5400584356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4376,"record_offset":1676,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":4376,"record_offset":1760,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4376,"record_offset":1976,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4376,"record_offset":1304,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4376,"record_offset":1588,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4376,"record_offset":1504,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1128,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1128,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1128,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":1128,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1128,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2104,"parent_record_offset":2048,"ancestor_record_offset":1128,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4376,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1128,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4376,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":1128,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4376,"record_offset":2076,"parent_record_offset":2048,"ancestor_record_offset":1128,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4376,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1128,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4376,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":1128,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4376,"record_offset":1616,"parent_record_offset":1588,"ancestor_record_offset":1128,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4376,"record_offset":1372,"parent_record_offset":1332,"ancestor_record_offset":1128,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4376,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1128,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4376,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1128,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4376,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1128,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4376,"record_offset":1400,"parent_record_offset":1332,"ancestor_record_offset":1128,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4376,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":1128,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4376,"record_offset":1436,"parent_record_offset":1400,"ancestor_record_offset":1128,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4376,"record_offset":2380,"rva":31875616,"va":5400584736}
// 编译实例 RVAs: 0x1e66220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27835106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e81e4557fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400584797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27835453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e8c34357fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400585160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2393043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e8d3832400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400585096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e83b0c7ffe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400584928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e8940c7ffe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400585031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e87b097ffe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400584912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e8820b7ffe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400584969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25228397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400584736,"raw_hex":"e8930b7ffe","size":5,"soft_fail":false,"target_va":5375356608,"va":5400585000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2560,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":3264,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4376,"record_offset":2708,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4376,"record_offset":3348,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2380,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":2380,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":2380,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":2592,"parent_record_offset":2560,"ancestor_record_offset":2380,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4376,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":2380,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4376,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":2380,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4376,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":2380,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4376,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2380,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4376,"record_offset":3140,"parent_record_offset":2968,"ancestor_record_offset":2380,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":2904,"parent_record_offset":2876,"ancestor_record_offset":2380,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2380,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":2380,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":2380,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":2380,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4376,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":2380,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4376,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2380,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4376,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2380,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4376,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2380,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4376,"record_offset":2940,"parent_record_offset":2904,"ancestor_record_offset":2380,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4376,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2380,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4376,"record_offset":3176,"parent_record_offset":2904,"ancestor_record_offset":2380,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4376,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":2380,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::calculate_move_dst
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4376,"record_offset":3464,"rva":31876064,"va":5400585184}
// 编译实例 RVAs: 0x1e663e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30971807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e8616827fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5400585322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2393264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e8b0842400","size":5,"soft_fail":false,"target_va":5402978912,"va":5400585643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2393220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e884842400","size":5,"soft_fail":false,"target_va":5402978880,"va":5400585655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25868501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e82b4775fe","size":5,"soft_fail":false,"target_va":5374716784,"va":5400585280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18652194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e8de63e3fe","size":5,"soft_fail":false,"target_va":5381933344,"va":5400585533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18328968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585184,"raw_hex":"e87852e8fe","size":5,"soft_fail":false,"target_va":5382256592,"va":5400585555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4376,"record_offset":3612,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":4376,"record_offset":3648,"parent_record_offset":3464,"ancestor_record_offset":3464,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4376,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":3464,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4376,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":3464,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4376,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3464,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4376,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3464,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":4376,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":3464,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4376,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":3464,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4376,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":3464,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4376,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":3464,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":4376,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":3464,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4376,"record_offset":3712,"parent_record_offset":3676,"ancestor_record_offset":3464,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4376,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":3464,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4376,"record_offset":4420,"parent_record_offset":4200,"ancestor_record_offset":3464,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4376,"record_offset":3764,"parent_record_offset":3736,"ancestor_record_offset":3464,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3464,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4308,"parent_record_offset":4200,"ancestor_record_offset":3464,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3464,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4628,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3464,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":3464,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":3792,"parent_record_offset":3736,"ancestor_record_offset":3464,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":3464,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4376,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":3464,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4376,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3464,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4376,"record_offset":4168,"parent_record_offset":4052,"ancestor_record_offset":3464,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4376,"record_offset":4052,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":3884,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4376,"record_offset":4200,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4376,"record_offset":3912,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4376,"record_offset":3996,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4376,"record_offset":4536,"parent_record_offset":3840,"ancestor_record_offset":3464,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/04376.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":4376,"record_offset":3676,"parent_record_offset":3648,"ancestor_record_offset":3464,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/04376.jsonl.gz"}
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
// 完整结构及原始类型记录: move_ops.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: move_ops.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: move_ops.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::calculate_move_dst
// 信息 ID: 4838a1c95ea7290dce312069
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6e41e -> 0x1e663e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x1c786cf -> 0x1e663e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier
// 信息 ID: 8bc58934b1f68e78f65475bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c7786b -> 0x1e66010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure$0
// 信息 ID: 45d04a0c24606feff0769ec6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::move_ops::show_decorator_remove_signifier", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0 <- core::ptr::drop_in_place<system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e65fc2 -> 0x1e66220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
