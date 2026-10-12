// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/glade-border/src/display_glade_border.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b5a7abcab456ce11b3daf6e83b2f0be0186a48b5b789583bc65c3e999fd1aa4c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: display_glade_border.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2134, 3399
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2134,"record_offset":1540,"rva":21178768,"va":5389887888}
// 编译实例 RVAs: 0x1432990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17138518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8aa7cfafe","size":5,"soft_fail":false,"target_va":5372749696,"va":5389888209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17138306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e87e7dfafe","size":5,"soft_fail":false,"target_va":5372749712,"va":5389888013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14007427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e87d432aff","size":5,"soft_fail":false,"target_va":5375881440,"va":5389888862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a96f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6702441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e869456600","size":5,"soft_fail":false,"target_va":5396590432,"va":5389887986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13705110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8961fd100","size":5,"soft_fail":false,"target_va":5403593360,"va":5389888245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13089363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e853bac700","size":5,"soft_fail":false,"target_va":5402978144,"va":5389888776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12358671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8f16b43ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5389888554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18583391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8a170e4fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5389888442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12361675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8356043ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5389888790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14532935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8b93e22ff","size":5,"soft_fail":false,"target_va":5375355744,"va":5389888674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7615083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e895cd8bff","size":5,"soft_fail":false,"target_va":5382273632,"va":5389888710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14164130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e85edf27ff","size":5,"soft_fail":false,"target_va":5375723824,"va":5389887949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5687849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8d735a9ff","size":5,"soft_fail":false,"target_va":5384200464,"va":5389888308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5687733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e84b36a9ff","size":5,"soft_fail":false,"target_va":5384200608,"va":5389888336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5687882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8b635a9ff","size":5,"soft_fail":false,"target_va":5384200400,"va":5389888277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::set_ui_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5687852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e8d435a9ff","size":5,"soft_fail":false,"target_va":5384200512,"va":5389888359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5687730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389887888,"raw_hex":"e84e36a9ff","size":5,"soft_fail":false,"target_va":5384200656,"va":5389888381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2134,"record_offset":3456,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2134,"record_offset":3540,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":2700,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":2332,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":3200,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":2608,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2134,"record_offset":1804,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":1888,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2134,"record_offset":1720,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1540,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1540,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1540,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":3608,"parent_record_offset":3572,"ancestor_record_offset":1540,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":2968,"parent_record_offset":2876,"ancestor_record_offset":1540,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":2036,"parent_record_offset":1948,"ancestor_record_offset":1540,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2134,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":1540,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2134,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":1540,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1540,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":1540,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":1540,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":1540,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":3088,"parent_record_offset":2848,"ancestor_record_offset":1540,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":1948,"parent_record_offset":1928,"ancestor_record_offset":1540,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":1540,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":1540,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":1540,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":3360,"parent_record_offset":3320,"ancestor_record_offset":1540,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":3320,"parent_record_offset":3284,"ancestor_record_offset":1540,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":2820,"parent_record_offset":2784,"ancestor_record_offset":1540,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1540,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2134,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":1540,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2134,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":1540,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1540,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":3284,"parent_record_offset":3200,"ancestor_record_offset":1540,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":2396,"parent_record_offset":2356,"ancestor_record_offset":1540,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2134,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1540,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":2134,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1540,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2134,"record_offset":2912,"parent_record_offset":2876,"ancestor_record_offset":1540,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2134,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":1540,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2134,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":1540,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2134,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1540,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2134,"record_offset":2468,"parent_record_offset":2420,"ancestor_record_offset":1540,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2134,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":1540,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2134,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":1540,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2134,"record_offset":2532,"parent_record_offset":2420,"ancestor_record_offset":1540,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2134,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":1540,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2134,"record_offset":3128,"parent_record_offset":2784,"ancestor_record_offset":1540,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":1908,"parent_record_offset":1888,"ancestor_record_offset":1540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2134,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1540,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1540,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":3416,"parent_record_offset":3320,"ancestor_record_offset":1540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2134,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1540,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19FA0
// 首条位置: {"module":3399,"inlinee":106400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2134,"record_offset":4656,"rva":21180032,"va":5389889152}
// 编译实例 RVAs: 0x1432e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17139528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8b878fafe","size":5,"soft_fail":false,"target_va":5372749696,"va":5389889219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17140019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8cd76fafe","size":5,"soft_fail":false,"target_va":5372749712,"va":5389889726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13704019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8531bd100","size":5,"soft_fail":false,"target_va":5403593360,"va":5389889336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13088493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8edb6c700","size":5,"soft_fail":false,"target_va":5402978144,"va":5389889646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13029583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8cfd0c600","size":5,"soft_fail":false,"target_va":5402919072,"va":5389889484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12362536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8d85c43ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5389889651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12642976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e860153fff","size":5,"soft_fail":false,"target_va":5377246528,"va":5389889499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14532708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e89c3f22ff","size":5,"soft_fail":false,"target_va":5375356704,"va":5389889407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14532547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e83d4022ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5389889438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14533396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8ec3c22ff","size":5,"soft_fail":false,"target_va":5375355984,"va":5389889375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14533124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389889152,"raw_hex":"e8fc3d22ff","size":5,"soft_fail":false,"target_va":5375356272,"va":5389889391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":4832,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":6028,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":4980,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2134,"record_offset":5480,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":6112,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":4656,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4656,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":4656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":4656,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2134,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":4656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2134,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4656,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5708,"parent_record_offset":5680,"ancestor_record_offset":4656,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5680,"parent_record_offset":5648,"ancestor_record_offset":4656,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":4656,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5156,"parent_record_offset":5128,"ancestor_record_offset":4656,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5128,"parent_record_offset":5100,"ancestor_record_offset":4656,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2134,"record_offset":5368,"parent_record_offset":5128,"ancestor_record_offset":4656,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":4656,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":4656,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":5036,"parent_record_offset":5008,"ancestor_record_offset":4656,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2134,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":4656,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":4656,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":5756,"parent_record_offset":5592,"ancestor_record_offset":4656,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":5620,"parent_record_offset":5592,"ancestor_record_offset":4656,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2134,"record_offset":5100,"parent_record_offset":5064,"ancestor_record_offset":4656,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2134,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":4656,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2134,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":4656,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2134,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":4656,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":2134,"record_offset":5788,"parent_record_offset":5556,"ancestor_record_offset":4656,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":2134,"record_offset":5592,"parent_record_offset":5556,"ancestor_record_offset":4656,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2134,"record_offset":5192,"parent_record_offset":5156,"ancestor_record_offset":4656,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2134,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":4656,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2134,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":4656,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2134,"record_offset":5408,"parent_record_offset":5064,"ancestor_record_offset":4656,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02134.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2134,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4656,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02134.jsonl.gz"}
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
// 完整结构及原始类型记录: display_glade_border.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: display_glade_border.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: display_glade_border.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>", "bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border
// 信息 ID: a7bdbfe4401b29bc0d090fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 11fed53679c9bf1381d011f5: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>)> @ 0x991d2e -> 0x1432990 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>)> > @ 0x143498a -> 0x1432990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$0
// 信息 ID: 8fad57441791771b9bf77dec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; module=3399, inline_record=1100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$1
// 信息 ID: d12b45daf0ac50df12805ded
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ptr::drop_in_place<system_glade_border::display_glade_border::display_glade_border::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1432913 -> 0x1432e80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
