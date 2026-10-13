// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5d217b743c63ff422015515c5ea85b762c3c58120678888f8734ea1f82546049
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_bottom_layer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1218, 2696
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
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1218,"record_offset":260,"rva":13230992,"va":5381940112}
// 编译实例 RVAs: 0xc9e390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12126819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e89df546ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5381940366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9191488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8c0bf73ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381941195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7879337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e857c587ff","size":5,"soft_fail":false,"target_va":5374061056,"va":5381940388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8809dffff","size":5,"soft_fail":false,"target_va":5381915200,"va":5381940411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141773930
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11358605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e88d51ad00","size":5,"soft_fail":false,"target_va":5393299760,"va":5381941150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21652864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e880654a01","size":5,"soft_fail":false,"target_va":5403593344,"va":5381940475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21653279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e81f674a01","size":5,"soft_fail":false,"target_va":5403593968,"va":5381940684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21037264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8d0004101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381941611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21113020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8bc284201","size":5,"soft_fail":false,"target_va":5403054624,"va":5381941599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21037808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8f0024101","size":5,"soft_fail":false,"target_va":5402979152,"va":5381941339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21037476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e8a4014101","size":5,"soft_fail":false,"target_va":5402978944,"va":5381941463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t21028410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e83ade4001","size":5,"soft_fail":false,"target_va":5402968992,"va":5381940577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t21029482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e86ae24001","size":5,"soft_fail":false,"target_va":5402970800,"va":5381941313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3973057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e83f60c3ff","size":5,"soft_fail":false,"target_va":5377967920,"va":5381940972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3971807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e82165c3ff","size":5,"soft_fail":false,"target_va":5377968464,"va":5381940266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5001928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381940112,"raw_hex":"e838adb3ff","size":5,"soft_fail":false,"target_va":5376938912,"va":5381940835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381940112,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5381940495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1218,"record_offset":1280,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1218,"record_offset":1520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1218,"record_offset":728,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1218,"record_offset":2004,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1218,"record_offset":1316,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1218,"record_offset":488,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1218,"record_offset":764,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1218,"record_offset":1400,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1218,"record_offset":1252,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1218,"record_offset":1584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1218,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1218,"record_offset":2156,"parent_record_offset":2056,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1218,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1218,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1218,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1218,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1218,"record_offset":556,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1218,"record_offset":1736,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1218,"record_offset":1764,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1218,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1218,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1218,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1218,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1218,"record_offset":1624,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1218,"record_offset":1052,"parent_record_offset":764,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1218,"record_offset":836,"parent_record_offset":764,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1218,"record_offset":1092,"parent_record_offset":764,"ancestor_record_offset":260,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1218,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1218,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1218,"record_offset":1480,"parent_record_offset":1448,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1218,"record_offset":960,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1218,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":260,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1218,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":260,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1218,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1218,"record_offset":1020,"parent_record_offset":764,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1218,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1218,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1218,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1218,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":260,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1218,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1218,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":260,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1218,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1218,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1218,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":260,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1218,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":260,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1218,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1218,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1218,"record_offset":872,"parent_record_offset":764,"ancestor_record_offset":260,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1218,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":260,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01218.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15DE7
// 首条位置: {"module":2696,"inlinee":89575}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2696,"record_offset":260,"rva":24590640,"va":5393299760}
// 编译实例 RVAs: 0x1773930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23486770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8ce9e99fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5393300317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16356122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8e66c06ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5393300085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16356235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8756c06ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5393300230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11430069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e84b9751ff","size":5,"soft_fail":false,"target_va":5381870240,"va":5393300304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11381538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8de5452ff","size":5,"soft_fail":false,"target_va":5381918976,"va":5393300509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7449858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8fe528eff","size":5,"soft_fail":false,"target_va":5385850032,"va":5393299885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2696,"record_offset":1324,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2696,"record_offset":1912,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::instanced_wall::impl$34::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":2696,"record_offset":1856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":63555,"source_line":38,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":1520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":1104,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":1408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":904,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":1016,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":960,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2696,"record_offset":612,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":2696,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":260,"inlinee":11220,"source_line":439,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2696,"record_offset":1572,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2696,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":664,"parent_record_offset":612,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":720,"parent_record_offset":612,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":260,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":260,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":832,"parent_record_offset":776,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":1160,"parent_record_offset":1128,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2696,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs
//     evidence={"module":2696,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":89575,"source_line":53,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2696,"record_offset":1972,"rva":24590640,"va":5393299760}
// 编译实例 RVAs: 0x1773930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23486770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8ce9e99fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5393300317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16356122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8e66c06ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5393300085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16356235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8756c06ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5393300230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11430069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e84b9751ff","size":5,"soft_fail":false,"target_va":5381870240,"va":5393300304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_bottom_layer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11381538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8de5452ff","size":5,"soft_fail":false,"target_va":5381918976,"va":5393300509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7449858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393299760,"raw_hex":"e8fe528eff","size":5,"soft_fail":false,"target_va":5385850032,"va":5393299885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2696,"record_offset":3036,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2696,"record_offset":3624,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::instanced_wall::impl$34::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":2696,"record_offset":3568,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":63555,"source_line":38,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":3232,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2464,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2816,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":3120,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2616,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2728,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2672,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2696,"record_offset":2324,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":2696,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":1972,"inlinee":11220,"source_line":439,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2696,"record_offset":3284,"parent_record_offset":3232,"ancestor_record_offset":1972,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2696,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":1972,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":2376,"parent_record_offset":2324,"ancestor_record_offset":1972,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":2432,"parent_record_offset":2324,"ancestor_record_offset":1972,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":1972,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2696,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1972,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1972,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2696,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1972,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2544,"parent_record_offset":2488,"ancestor_record_offset":1972,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2872,"parent_record_offset":2840,"ancestor_record_offset":1972,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":1972,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2696,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1972,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2696,"record_offset":2576,"parent_record_offset":2544,"ancestor_record_offset":1972,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02696.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_bottom_layer.rs
//     evidence={"module":2696,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":1972,"inlinee":89576,"source_line":83,"archive":"_pdb/A/modules/02696.jsonl.gz"}
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
// 完整结构及原始类型记录: circle_bottom_layer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_bottom_layer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_bottom_layer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> >
// 信息 ID: 3b85226a7c2f1d9802f37ba1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick -> ["ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e79e -> 0x1773930 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2c4d -> 0x1773930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1> >
// 信息 ID: 7684e3ac655896d78e128dbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick -> ["ref_mut$<system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e79e -> 0x1773930 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2c4d -> 0x1773930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer
// 信息 ID: 56159f97f7f97bd2b4bbda0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x4d56e1 -> 0xc9e390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure$0
// 信息 ID: 864983b1b81a29e78b147287
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0 <- system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> >; module=2696, inline_record=1936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
