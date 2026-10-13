// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/history/wall_history.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 04207575069dccdcb719c66b33ba49dfb9c0e0cc00b3f2c9667c8926369f6991
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_history.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 444, 496
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":444,"record_offset":6184,"rva":6251552,"va":5374960672}
// 编译实例 RVAs: 0x5f6420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2211400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8b841deff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374961091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2212899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8dd3bdeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374962606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t917951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8bf010e00","size":5,"soft_fail":false,"target_va":5375881440,"va":5374963484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t941176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8785c0e00","size":5,"soft_fail":false,"target_va":5375904144,"va":5374962963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ef330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e898860f00","size":5,"soft_fail":false,"target_va":5375980336,"va":5374962835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9005863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8276b8900","size":5,"soft_fail":false,"target_va":5383966736,"va":5374960868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28632205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e88de4b401","size":5,"soft_fail":false,"target_va":5403593360,"va":5374961150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t28017023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e87f81ab01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374961116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2564621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e80d222700","size":5,"soft_fail":false,"target_va":5377527120,"va":5374962494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28016998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e86681ab01","size":5,"soft_fail":false,"target_va":5402979152,"va":5374962149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28016673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e82180ab01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374962266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e892f4ffff","size":5,"soft_fail":false,"target_va":5374960288,"va":5374963209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e8bf100000","size":5,"soft_fail":false,"target_va":5374966368,"va":5374962076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-142736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374960672,"raw_hex":"e870d2fdff","size":5,"soft_fail":false,"target_va":5374819280,"va":5374962011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7968,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":444,"record_offset":9120,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":8536,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":444,"record_offset":8272,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":444,"record_offset":6392,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":444,"record_offset":6628,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":444,"record_offset":6364,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":444,"record_offset":6420,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6184,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6184,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":6184,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":8348,"parent_record_offset":8312,"ancestor_record_offset":6184,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7016,"parent_record_offset":6916,"ancestor_record_offset":6184,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":7892,"parent_record_offset":7800,"ancestor_record_offset":6184,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":444,"record_offset":8312,"parent_record_offset":8272,"ancestor_record_offset":6184,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":6184,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6184,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":6884,"parent_record_offset":6852,"ancestor_record_offset":6184,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":7136,"parent_record_offset":6884,"ancestor_record_offset":6184,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":6184,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":6812,"parent_record_offset":6780,"ancestor_record_offset":6184,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":8560,"parent_record_offset":8536,"ancestor_record_offset":6184,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":6184,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":6748,"parent_record_offset":6712,"ancestor_record_offset":6184,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":6184,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":6852,"parent_record_offset":6812,"ancestor_record_offset":6184,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":6712,"parent_record_offset":6664,"ancestor_record_offset":6184,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":444,"record_offset":6960,"parent_record_offset":6916,"ancestor_record_offset":6184,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":444,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":6184,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":6184,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6184,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":444,"record_offset":6568,"parent_record_offset":6420,"ancestor_record_offset":6184,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":444,"record_offset":6596,"parent_record_offset":6420,"ancestor_record_offset":6184,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":444,"record_offset":7204,"parent_record_offset":7180,"ancestor_record_offset":6184,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":444,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":6184,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":444,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":6184,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":444,"record_offset":6460,"parent_record_offset":6420,"ancestor_record_offset":6184,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":444,"record_offset":7180,"parent_record_offset":6812,"ancestor_record_offset":6184,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":7752,"parent_record_offset":6664,"ancestor_record_offset":6184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":7092,"parent_record_offset":7064,"ancestor_record_offset":6184,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":444,"record_offset":6664,"parent_record_offset":6628,"ancestor_record_offset":6184,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x63DB
// 首条位置: {"module":496,"inlinee":25563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::EditType::new_height_changed
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":444,"record_offset":9376,"rva":6254576,"va":5374963696}
// 编译实例 RVAs: 0x5f6ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2214080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e84037deff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374963771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2214423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e8e935deff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374964130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ef330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1016135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e847810f00","size":5,"soft_fail":false,"target_va":5375980336,"va":5374964196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28629452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e8ccd9b401","size":5,"soft_fail":false,"target_va":5403593360,"va":5374963903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28014124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e82c76ab01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374964015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-144679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374963696,"raw_hex":"e8d9cafdff","size":5,"soft_fail":false,"target_va":5374819280,"va":5374963954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":9536,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":444,"record_offset":10172,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":10088,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":444,"record_offset":9832,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":9376,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":9376,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":9376,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":9900,"parent_record_offset":9864,"ancestor_record_offset":9376,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":9568,"parent_record_offset":9536,"ancestor_record_offset":9376,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":444,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":9376,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":444,"record_offset":9864,"parent_record_offset":9832,"ancestor_record_offset":9376,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":444,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":9376,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":9376,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":10052,"parent_record_offset":9864,"ancestor_record_offset":9376,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":9640,"parent_record_offset":9616,"ancestor_record_offset":9376,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00444.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_history.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_history.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_history.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::history::EditType::new_height_changed
// 信息 ID: 69672110776023e2db94db67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::deferred_wall_history::history_record_merge_walls @ 0x6b6a4e -> 0x5f6ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height @ 0x6c712c -> 0x5f6ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 信息 ID: 1fab0af5c65fcfb347ecde31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height; module=496, inline_record=26488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 信息 ID: a908367797ffc96d63266fbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0x6815df -> 0x5f6420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x69f0a5 -> 0x5f6420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0x112de64 -> 0x5f6420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
