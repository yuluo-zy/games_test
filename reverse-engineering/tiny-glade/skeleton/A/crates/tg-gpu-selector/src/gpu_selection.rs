// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-gpu-selector/src/gpu_selection.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a39711b49d6467a9a4d30b8b11c7ea655902327bb80ef5d85d7d3606b21f4b8b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gpu_selection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 256, 269, 779, 1983, 3235, 3244, 3246, 3248
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: []
// use iced_runtime; // namespace candidate
//   package identities: []
// use iced_widget; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tg_iced; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::device_driver_advice
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":11952,"rva":26839376,"va":5395548496}
// 编译实例 RVAs: 0x1998950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=15
//     target_aliases=101 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26561501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e823b46afe","size":5,"soft_fail":false,"target_va":5368995152,"va":5395556648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=7
//     target_aliases=4 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25916274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e88e8c74fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5395548717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=15
//     target_aliases=31 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23321578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e816249cfe","size":5,"soft_fail":false,"target_va":5372235632,"va":5395557205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=36
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22799100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8041da4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395548791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=36
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22799062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e82a1da4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395548769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9350
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17699081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8f7eef1fe","size":5,"soft_fail":false,"target_va":5377856336,"va":5395555412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17667941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e89b68f2fe","size":5,"soft_fail":false,"target_va":5377881008,"va":5395548944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fa1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6944365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8930996ff","size":5,"soft_fail":false,"target_va":5388607920,"va":5395552280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8043912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e888bd7a00","size":5,"soft_fail":false,"target_va":5403593360,"va":5395549443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=36
//     evidence={"assembly":"\tcallq\t7427045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8e5537100","size":5,"soft_fail":false,"target_va":5402978144,"va":5395551094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-18018819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8fd0dedfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395548702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t7369765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e825747000","size":5,"soft_fail":false,"target_va":5402919072,"va":5395549302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-17664911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e87174f2fe","size":5,"soft_fail":false,"target_va":5377885840,"va":5395550746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17662529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8bf7df2fe","size":5,"soft_fail":false,"target_va":5377891056,"va":5395553580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-17648209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8afb5f2fe","size":5,"soft_fail":false,"target_va":5377902688,"va":5395550892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-19395681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e89f0bd8fe","size":5,"soft_fail":false,"target_va":5376152880,"va":5395548556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19294293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395548496,"raw_hex":"e8ab97d9fe","size":5,"soft_fail":false,"target_va":5376254928,"va":5395549216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":23684,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":33796,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":12160,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":25000,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":21816,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":13820,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":13848,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":22848,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3235,"record_offset":13504,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":16592,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::style
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":17320,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":45522,"source_line":77,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":15004,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":18340,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::max_width
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":13476,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":77333,"source_line":97,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":3235,"record_offset":22904,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14478,"source_line":68,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":3235,"record_offset":12776,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14501,"source_line":57,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":3235,"record_offset":18640,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":3235,"record_offset":17844,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":3235,"record_offset":12244,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":11952,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":11952,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":11952,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":13020,"parent_record_offset":12980,"ancestor_record_offset":11952,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":13052,"parent_record_offset":13020,"ancestor_record_offset":11952,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":12496,"parent_record_offset":12440,"ancestor_record_offset":11952,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":3235,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":11952,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":12980,"parent_record_offset":12944,"ancestor_record_offset":11952,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":32756,"parent_record_offset":32728,"ancestor_record_offset":11952,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":11952,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":11952,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":11952,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":14284,"parent_record_offset":14256,"ancestor_record_offset":11952,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":14048,"parent_record_offset":14012,"ancestor_record_offset":11952,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":11952,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":21992,"parent_record_offset":21964,"ancestor_record_offset":11952,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":21964,"parent_record_offset":21936,"ancestor_record_offset":11952,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":22140,"parent_record_offset":21964,"ancestor_record_offset":11952,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":12440,"parent_record_offset":12412,"ancestor_record_offset":11952,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":11952,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":25024,"parent_record_offset":25000,"ancestor_record_offset":11952,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":21872,"parent_record_offset":21844,"ancestor_record_offset":11952,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":21844,"parent_record_offset":21816,"ancestor_record_offset":11952,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":12280,"parent_record_offset":12244,"ancestor_record_offset":11952,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":11952,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":14012,"parent_record_offset":13920,"ancestor_record_offset":11952,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":11952,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":25048,"parent_record_offset":25024,"ancestor_record_offset":11952,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":14080,"parent_record_offset":13920,"ancestor_record_offset":11952,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":13956,"parent_record_offset":13920,"ancestor_record_offset":11952,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":21936,"parent_record_offset":21900,"ancestor_record_offset":11952,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3235,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":11952,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3235,"record_offset":12872,"parent_record_offset":12812,"ancestor_record_offset":11952,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3235,"record_offset":22200,"parent_record_offset":22176,"ancestor_record_offset":11952,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3235,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":11952,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3235,"record_offset":35476,"parent_record_offset":35448,"ancestor_record_offset":11952,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3235,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":11952,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3235,"record_offset":22176,"parent_record_offset":21900,"ancestor_record_offset":11952,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=123
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":12364,"parent_record_offset":12244,"ancestor_record_offset":11952,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3235,"record_offset":22872,"parent_record_offset":22848,"ancestor_record_offset":11952,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":11952,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=32
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":3235,"record_offset":12944,"parent_record_offset":12908,"ancestor_record_offset":11952,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":3235,"record_offset":13412,"parent_record_offset":12812,"ancestor_record_offset":11952,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":12336,"parent_record_offset":12244,"ancestor_record_offset":11952,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":12908,"parent_record_offset":12872,"ancestor_record_offset":11952,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":23008,"parent_record_offset":22928,"ancestor_record_offset":11952,"inlinee":14483,"source_line":145,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":11952,"inlinee":14482,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":22952,"parent_record_offset":22928,"ancestor_record_offset":11952,"inlinee":14481,"source_line":32,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":22928,"parent_record_offset":22904,"ancestor_record_offset":11952,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":18968,"parent_record_offset":18940,"ancestor_record_offset":11952,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":12812,"parent_record_offset":12776,"ancestor_record_offset":11952,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":13540,"parent_record_offset":13504,"ancestor_record_offset":11952,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":3235,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":11952,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":392,"rva":26827104,"va":5395536224}
// 编译实例 RVAs: 0x1995960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23222242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536224,"raw_hex":"e81ea89dfe","size":5,"soft_fail":false,"target_va":5372314208,"va":5395536445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4036887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536224,"raw_hex":"e817993d00","size":5,"soft_fail":false,"target_va":5399573248,"va":5395536356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19382353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536224,"raw_hex":"e8af3fd8fe","size":5,"soft_fail":false,"target_va":5376154000,"va":5395536348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18117641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536224,"raw_hex":"e8f78bebfe","size":5,"soft_fail":false,"target_va":5377418800,"va":5395536436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19543050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536224,"raw_hex":"e8f6cbd5fe","size":5,"soft_fail":false,"target_va":5375993248,"va":5395536293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3235,"record_offset":620,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3235,"record_offset":536,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":3235,"record_offset":648,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":14470,"source_line":30,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/window.rs
//     evidence={"module":3235,"record_offset":564,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":14461,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":3235,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":392,"inlinee":14462,"source_line":35,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":256,"record_offset":15084,"rva":3953248,"va":5372662368}
// 编译实例 RVAs: 0x3c5260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4923410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662368,"raw_hex":"e812204b00","size":5,"soft_fail":false,"target_va":5377585888,"va":5372662473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t4921482","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372662368,"raw_hex":"e98a184b00","size":5,"soft_fail":false,"target_va":5377583920,"va":5372662433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18D43
// 首条位置: {"module":3246,"inlinee":101699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::clone::impls::impl$3::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3246,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":32536,"source_line":263,"archive":"_pdb/A/modules/03246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$0::program_outcome
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB3D6
// 首条位置: {"module":779,"inlinee":46038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::Controls::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3993
// 首条位置: {"module":269,"inlinee":14739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":269,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":708,"rva":26827392,"va":5395536512}
// 编译实例 RVAs: 0x1995a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17951397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e85b15eefe","size":5,"soft_fail":false,"target_va":5377588464,"va":5395539856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=11
//     target_aliases=101 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26549072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8b0e46afe","size":5,"soft_fail":false,"target_va":5368995152,"va":5395544219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=8
//     target_aliases=31 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23308498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e82e579cfe","size":5,"soft_fail":false,"target_va":5372235632,"va":5395544125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22787688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e89849a4fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395537379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22793208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e80834a4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395542915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18103007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e821c5ebfe","size":5,"soft_fail":false,"target_va":5377436496,"va":5395539498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9350
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17688000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8401af2fe","size":5,"soft_fail":false,"target_va":5377856336,"va":5395544331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9490
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17690317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e83311f2fe","size":5,"soft_fail":false,"target_va":5377856656,"va":5395546968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3b0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17659050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8568bf2fe","size":5,"soft_fail":false,"target_va":5377881008,"va":5395540053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17661665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e81f81f2fe","size":5,"soft_fail":false,"target_va":5377881024,"va":5395542684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fc320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6921247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8e16396ff","size":5,"soft_fail":false,"target_va":5388616480,"va":5395537722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fc8b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6920946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e80e6596ff","size":5,"soft_fail":false,"target_va":5388617904,"va":5395538845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141301170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6902434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e85ead96ff","size":5,"soft_fail":false,"target_va":5388636528,"va":5395538957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8055939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e883ec7a00","size":5,"soft_fail":false,"target_va":5403593360,"va":5395537416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t7434946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8c2727100","size":5,"soft_fail":false,"target_va":5402978144,"va":5395543193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18007435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8753aedfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395537318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-24233603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e87d398efe","size":5,"soft_fail":false,"target_va":5371304880,"va":5395538478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18016603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8a516edfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5395543718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7436173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e88d777100","size":5,"soft_fail":false,"target_va":5402979344,"va":5395543166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_gpu_selector..gpu_selection..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-17461214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e82290f5fe","size":5,"soft_fail":false,"target_va":5378084112,"va":5395545321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..image..Image$LT$iced_core..image..Handle$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-22643760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8d07ba6fe","size":5,"soft_fail":false,"target_va":5372901888,"va":5395545643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..gpu_selection..LocalizedError$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17455881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8f7a4f5fe","size":5,"soft_fail":false,"target_va":5378087984,"va":5395543860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7436842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e82a7a7100","size":5,"soft_fail":false,"target_va":5402980128,"va":5395543281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::image::Handle::from_data
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22581564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8c46ea7fe","size":5,"soft_fail":false,"target_va":5372957968,"va":5395539527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17656902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8ba93f2fe","size":5,"soft_fail":false,"target_va":5377885328,"va":5395542225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17648277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e86bb5f2fe","size":5,"soft_fail":false,"target_va":5377891056,"va":5395539328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17638229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8abdcf2fe","size":5,"soft_fail":false,"target_va":5377902688,"va":5395540912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-19383791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8113ad8fe","size":5,"soft_fail":false,"target_va":5376152880,"va":5395536666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19284771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e8ddbcd9fe","size":5,"soft_fail":false,"target_va":5376254928,"va":5395539694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::gpu_selection::localize_graphics_init_error
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t28800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e880700000","size":5,"soft_fail":false,"target_va":5395565536,"va":5395536731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::gpu_selection::render_error_to_ui
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t24411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395536512,"raw_hex":"e85b5f0000","size":5,"soft_fail":false,"target_va":5395562464,"va":5395538048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*32(%r13)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5395536512,"raw_hex":"41ff5520","size":4,"soft_fail":false,"va":5395538083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":1672,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":11780,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":1080,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":2364,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3235,"record_offset":1164,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":2916,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":4032,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3235,"record_offset":2024,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":10740,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3235,"record_offset":4128,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3235,"record_offset":968,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3235,"record_offset":1052,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":9155,"source_line":187,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":3235,"record_offset":6352,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14495,"source_line":112,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":3235,"record_offset":6380,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":1992,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14484,"source_line":100,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":3744,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14535,"source_line":76,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":3236,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":8912,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":10360,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14506,"source_line":121,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_y
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":10388,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14507,"source_line":127,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::max_width
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":2548,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":77333,"source_line":97,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":2448,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::padding
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":2576,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":76793,"source_line":79,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":3235,"record_offset":4604,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":3235,"record_offset":1824,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14478,"source_line":68,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":3235,"record_offset":9956,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14501,"source_line":57,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::Image::height
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/image.rs
//     evidence={"module":3235,"record_offset":4004,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":101643,"source_line":62,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::Image::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/image.rs
//     evidence={"module":3235,"record_offset":3804,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":101639,"source_line":45,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/image.rs
//     evidence={"module":3235,"record_offset":8640,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":77342,"source_line":217,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":3235,"record_offset":6924,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":3235,"record_offset":9212,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":3235,"record_offset":9572,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":14500,"source_line":51,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":3235,"record_offset":4156,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":101645,"source_line":414,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":3235,"record_offset":4184,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":708,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":708,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":708,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":2168,"parent_record_offset":2132,"ancestor_record_offset":708,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":1432,"parent_record_offset":1396,"ancestor_record_offset":708,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":4524,"parent_record_offset":4468,"ancestor_record_offset":708,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":2132,"parent_record_offset":2096,"ancestor_record_offset":708,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":708,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":4496,"parent_record_offset":4468,"ancestor_record_offset":708,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":2992,"parent_record_offset":2960,"ancestor_record_offset":708,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":708,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":708,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":1552,"parent_record_offset":1368,"ancestor_record_offset":708,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":4468,"parent_record_offset":4440,"ancestor_record_offset":708,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":708,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":708,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":708,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":708,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":708,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":708,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":4272,"parent_record_offset":4220,"ancestor_record_offset":708,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":4220,"parent_record_offset":4184,"ancestor_record_offset":708,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":2960,"parent_record_offset":2916,"ancestor_record_offset":708,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":708,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":1340,"parent_record_offset":1304,"ancestor_record_offset":708,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3235,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":708,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3235,"record_offset":4300,"parent_record_offset":4220,"ancestor_record_offset":708,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3235,"record_offset":4328,"parent_record_offset":4220,"ancestor_record_offset":708,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3235,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":708,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3235,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":708,"inlinee":6075,"source_line":85,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3235,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":708,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3235,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":708,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3235,"record_offset":1592,"parent_record_offset":1304,"ancestor_record_offset":708,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":2320,"parent_record_offset":2132,"ancestor_record_offset":708,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3235,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":708,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":708,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":3024,"parent_record_offset":2916,"ancestor_record_offset":708,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3235,"record_offset":4356,"parent_record_offset":4220,"ancestor_record_offset":708,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3235,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":708,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3235,"record_offset":4088,"parent_record_offset":4060,"ancestor_record_offset":708,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3235,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":708,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":3235,"record_offset":2096,"parent_record_offset":2060,"ancestor_record_offset":708,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::Handle::from_path
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/image.rs
//     evidence={"module":3235,"record_offset":3964,"parent_record_offset":3856,"ancestor_record_offset":708,"inlinee":101642,"source_line":19,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$1::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/image.rs
//     evidence={"module":3235,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":708,"inlinee":101641,"source_line":77,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":3235,"record_offset":2488,"parent_record_offset":2448,"ancestor_record_offset":708,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":4728,"parent_record_offset":4692,"ancestor_record_offset":708,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":3235,"record_offset":4640,"parent_record_offset":4604,"ancestor_record_offset":708,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":1928,"parent_record_offset":1848,"ancestor_record_offset":708,"inlinee":14483,"source_line":145,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":708,"inlinee":14482,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":708,"inlinee":14481,"source_line":32,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":708,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":2060,"parent_record_offset":2024,"ancestor_record_offset":708,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":9992,"parent_record_offset":9956,"ancestor_record_offset":708,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":3235,"record_offset":2640,"parent_record_offset":2604,"ancestor_record_offset":708,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::scrollable::impl$6::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":3235,"record_offset":10076,"parent_record_offset":10040,"ancestor_record_offset":708,"inlinee":14505,"source_line":420,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3235,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":708,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3235,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":708,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E09
// 首条位置: {"module":1983,"inlinee":77321}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":19772,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":19800,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1983,"record_offset":25972,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":31716,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::style
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":25944,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":45522,"source_line":77,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":26672,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":28184,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::radio
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":1983,"record_offset":19564,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":77322,"source_line":173,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::Radio::font
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/radio.rs
//     evidence={"module":1983,"record_offset":19744,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":77325,"source_line":175,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/radio.rs
//     evidence={"module":1983,"record_offset":27168,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":77327,"source_line":364,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":29084,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":28484,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":1983,"record_offset":27440,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":1983,"record_offset":20428,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":26140,"parent_record_offset":26116,"ancestor_record_offset":19228,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":26164,"parent_record_offset":26140,"ancestor_record_offset":19228,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":20704,"parent_record_offset":20680,"ancestor_record_offset":19228,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":26084,"parent_record_offset":26056,"ancestor_record_offset":19228,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":26116,"parent_record_offset":26084,"ancestor_record_offset":19228,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":20680,"parent_record_offset":20624,"ancestor_record_offset":19228,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":1983,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":19228,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1983,"record_offset":26056,"parent_record_offset":26028,"ancestor_record_offset":19228,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":19228,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":20652,"parent_record_offset":20624,"ancestor_record_offset":19228,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":20236,"parent_record_offset":20208,"ancestor_record_offset":19228,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":20000,"parent_record_offset":19964,"ancestor_record_offset":19228,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":19932,"parent_record_offset":19908,"ancestor_record_offset":19228,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":20624,"parent_record_offset":20596,"ancestor_record_offset":19228,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":21048,"parent_record_offset":20996,"ancestor_record_offset":19228,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":20996,"parent_record_offset":20960,"ancestor_record_offset":19228,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":20464,"parent_record_offset":20428,"ancestor_record_offset":19228,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$46::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":19648,"parent_record_offset":19624,"ancestor_record_offset":19228,"inlinee":6525,"source_line":2729,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":19872,"parent_record_offset":19848,"ancestor_record_offset":19228,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":19964,"parent_record_offset":19872,"ancestor_record_offset":19228,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":19824,"parent_record_offset":19800,"ancestor_record_offset":19228,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":20032,"parent_record_offset":19872,"ancestor_record_offset":19228,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":19908,"parent_record_offset":19872,"ancestor_record_offset":19228,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1983,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":19228,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1983,"record_offset":19624,"parent_record_offset":19592,"ancestor_record_offset":19228,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":21076,"parent_record_offset":20996,"ancestor_record_offset":19228,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":21104,"parent_record_offset":20996,"ancestor_record_offset":19228,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1983,"record_offset":19680,"parent_record_offset":19592,"ancestor_record_offset":19228,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1983,"record_offset":19704,"parent_record_offset":19680,"ancestor_record_offset":19228,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":20548,"parent_record_offset":20428,"ancestor_record_offset":19228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1983,"record_offset":25360,"parent_record_offset":25172,"ancestor_record_offset":19228,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":26192,"parent_record_offset":26164,"ancestor_record_offset":19228,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1983,"record_offset":21132,"parent_record_offset":20996,"ancestor_record_offset":19228,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":1983,"record_offset":26028,"parent_record_offset":26000,"ancestor_record_offset":19228,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":20520,"parent_record_offset":20428,"ancestor_record_offset":19228,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":30340,"parent_record_offset":30304,"ancestor_record_offset":19228,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::radio::Radio::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/radio.rs
//     evidence={"module":1983,"record_offset":19592,"parent_record_offset":19564,"ancestor_record_offset":19228,"inlinee":77324,"source_line":110,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":1983,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":19228,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E14
// 首条位置: {"module":1983,"inlinee":77332}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1983,"record_offset":48184,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1983,"record_offset":46924,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1983,"record_offset":43400,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14514,"source_line":271,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":50120,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1983,"record_offset":47840,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1983,"record_offset":50176,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":49604,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::style
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":48800,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":45522,"source_line":77,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":48828,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":48152,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":49328,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":50872,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::max_width
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":1983,"record_offset":47784,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":77333,"source_line":97,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":1983,"record_offset":47684,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::padding
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":1983,"record_offset":47812,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":76793,"source_line":79,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":1983,"record_offset":50232,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14478,"source_line":68,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::Radio::style
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/radio.rs
//     evidence={"module":1983,"record_offset":51204,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":77334,"source_line":181,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/radio.rs
//     evidence={"module":1983,"record_offset":52148,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":77327,"source_line":364,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":53600,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":52968,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":1983,"record_offset":52420,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":1983,"record_offset":46504,"parent_record_offset":43080,"ancestor_record_offset":43080,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":47196,"parent_record_offset":47172,"ancestor_record_offset":43080,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":47220,"parent_record_offset":47196,"ancestor_record_offset":43080,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":46452,"parent_record_offset":46428,"ancestor_record_offset":43080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":47984,"parent_record_offset":47948,"ancestor_record_offset":43080,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":47172,"parent_record_offset":47136,"ancestor_record_offset":43080,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":46428,"parent_record_offset":46372,"ancestor_record_offset":43080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":1983,"record_offset":48328,"parent_record_offset":48304,"ancestor_record_offset":43080,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1983,"record_offset":47948,"parent_record_offset":47912,"ancestor_record_offset":43080,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1983,"record_offset":48232,"parent_record_offset":48208,"ancestor_record_offset":43080,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":43832,"parent_record_offset":43808,"ancestor_record_offset":43080,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":46400,"parent_record_offset":46372,"ancestor_record_offset":43080,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":43808,"parent_record_offset":43780,"ancestor_record_offset":43080,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":43724,"parent_record_offset":43692,"ancestor_record_offset":43080,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":43780,"parent_record_offset":43756,"ancestor_record_offset":43080,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":47136,"parent_record_offset":47104,"ancestor_record_offset":43080,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":47104,"parent_record_offset":47072,"ancestor_record_offset":43080,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":47576,"parent_record_offset":47520,"ancestor_record_offset":43080,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":46372,"parent_record_offset":46344,"ancestor_record_offset":43080,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1983,"record_offset":47032,"parent_record_offset":46996,"ancestor_record_offset":43080,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1983,"record_offset":46996,"parent_record_offset":46960,"ancestor_record_offset":43080,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1983,"record_offset":46960,"parent_record_offset":46924,"ancestor_record_offset":43080,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":43484,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":43568,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":46540,"parent_record_offset":46504,"ancestor_record_offset":43080,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":48304,"parent_record_offset":48268,"ancestor_record_offset":43080,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":43652,"parent_record_offset":43624,"ancestor_record_offset":43080,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":43692,"parent_record_offset":43652,"ancestor_record_offset":43080,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":43596,"parent_record_offset":43568,"ancestor_record_offset":43080,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":43880,"parent_record_offset":43652,"ancestor_record_offset":43080,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":43756,"parent_record_offset":43652,"ancestor_record_offset":43080,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1983,"record_offset":47072,"parent_record_offset":47032,"ancestor_record_offset":43080,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1983,"record_offset":43624,"parent_record_offset":43596,"ancestor_record_offset":43080,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1983,"record_offset":49392,"parent_record_offset":49360,"ancestor_record_offset":43080,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1983,"record_offset":49420,"parent_record_offset":49360,"ancestor_record_offset":43080,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1983,"record_offset":49524,"parent_record_offset":49500,"ancestor_record_offset":43080,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":1983,"record_offset":45376,"parent_record_offset":44712,"ancestor_record_offset":43080,"inlinee":5382,"source_line":612,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\methods.rs
//     evidence={"module":1983,"record_offset":45400,"parent_record_offset":45376,"ancestor_record_offset":43080,"inlinee":5096,"source_line":1741,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1983,"record_offset":44384,"parent_record_offset":44348,"ancestor_record_offset":43080,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":46620,"parent_record_offset":46540,"ancestor_record_offset":43080,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":46648,"parent_record_offset":46540,"ancestor_record_offset":43080,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1983,"record_offset":47324,"parent_record_offset":47300,"ancestor_record_offset":43080,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":1983,"record_offset":49452,"parent_record_offset":49420,"ancestor_record_offset":43080,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1983,"record_offset":50200,"parent_record_offset":50176,"ancestor_record_offset":43080,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1983,"record_offset":44488,"parent_record_offset":44092,"ancestor_record_offset":43080,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44636,"parent_record_offset":44560,"ancestor_record_offset":43080,"inlinee":14451,"source_line":213,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44420,"parent_record_offset":44348,"ancestor_record_offset":43080,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44348,"parent_record_offset":44316,"ancestor_record_offset":43080,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44560,"parent_record_offset":44524,"ancestor_record_offset":43080,"inlinee":7279,"source_line":798,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44316,"parent_record_offset":44092,"ancestor_record_offset":43080,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1983,"record_offset":44524,"parent_record_offset":44488,"ancestor_record_offset":43080,"inlinee":7278,"source_line":967,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1983,"record_offset":49476,"parent_record_offset":49452,"ancestor_record_offset":43080,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1983,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":43080,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1983,"record_offset":44660,"parent_record_offset":44636,"ancestor_record_offset":43080,"inlinee":14452,"source_line":664,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1983,"record_offset":45024,"parent_record_offset":44912,"ancestor_record_offset":43080,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1983,"record_offset":49548,"parent_record_offset":49524,"ancestor_record_offset":43080,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":1983,"record_offset":49500,"parent_record_offset":49476,"ancestor_record_offset":43080,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1983,"record_offset":48208,"parent_record_offset":48184,"ancestor_record_offset":43080,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1983,"record_offset":47300,"parent_record_offset":47032,"ancestor_record_offset":43080,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":46296,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1983,"record_offset":50144,"parent_record_offset":50120,"ancestor_record_offset":43080,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1983,"record_offset":45220,"parent_record_offset":45176,"ancestor_record_offset":43080,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":47248,"parent_record_offset":47220,"ancestor_record_offset":43080,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1983,"record_offset":46676,"parent_record_offset":46540,"ancestor_record_offset":43080,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1983,"record_offset":44228,"parent_record_offset":44092,"ancestor_record_offset":43080,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1983,"record_offset":44268,"parent_record_offset":44228,"ancestor_record_offset":43080,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1983,"record_offset":45176,"parent_record_offset":45108,"ancestor_record_offset":43080,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1983,"record_offset":45452,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":12626,"source_line":507,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1983,"record_offset":44912,"parent_record_offset":44884,"ancestor_record_offset":43080,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1983,"record_offset":43512,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":14515,"source_line":1882,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1983,"record_offset":43968,"parent_record_offset":43924,"ancestor_record_offset":43080,"inlinee":14519,"source_line":1054,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1983,"record_offset":45072,"parent_record_offset":44712,"ancestor_record_offset":43080,"inlinee":4928,"source_line":42,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1983,"record_offset":43924,"parent_record_offset":43400,"ancestor_record_offset":43080,"inlinee":14517,"source_line":506,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1983,"record_offset":44092,"parent_record_offset":44012,"ancestor_record_offset":43080,"inlinee":14545,"source_line":1427,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1983,"record_offset":43536,"parent_record_offset":43512,"ancestor_record_offset":43080,"inlinee":7267,"source_line":956,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1983,"record_offset":44712,"parent_record_offset":44012,"ancestor_record_offset":43080,"inlinee":14454,"source_line":1085,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1983,"record_offset":44012,"parent_record_offset":43968,"ancestor_record_offset":43080,"inlinee":14546,"source_line":1137,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1983,"record_offset":44884,"parent_record_offset":44848,"ancestor_record_offset":43080,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1983,"record_offset":44848,"parent_record_offset":44812,"ancestor_record_offset":43080,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1983,"record_offset":44812,"parent_record_offset":44712,"ancestor_record_offset":43080,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1983,"record_offset":45476,"parent_record_offset":45452,"ancestor_record_offset":43080,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1983,"record_offset":45108,"parent_record_offset":45072,"ancestor_record_offset":43080,"inlinee":4930,"source_line":36,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1983,"record_offset":45284,"parent_record_offset":45108,"ancestor_record_offset":43080,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1983,"record_offset":45256,"parent_record_offset":45108,"ancestor_record_offset":43080,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":1983,"record_offset":47912,"parent_record_offset":47876,"ancestor_record_offset":43080,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":1983,"record_offset":47724,"parent_record_offset":47684,"ancestor_record_offset":43080,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":48360,"parent_record_offset":48268,"ancestor_record_offset":43080,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":49360,"parent_record_offset":49328,"ancestor_record_offset":43080,"inlinee":14483,"source_line":145,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":50304,"parent_record_offset":50280,"ancestor_record_offset":43080,"inlinee":14482,"source_line":50,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":50280,"parent_record_offset":50256,"ancestor_record_offset":43080,"inlinee":14481,"source_line":32,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":1983,"record_offset":50256,"parent_record_offset":50232,"ancestor_record_offset":43080,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":1983,"record_offset":47876,"parent_record_offset":47840,"ancestor_record_offset":43080,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E08
// 首条位置: {"module":1983,"inlinee":77320}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":1983,"record_offset":16164,"parent_record_offset":16132,"ancestor_record_offset":16132,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":16132,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":16496,"parent_record_offset":16440,"ancestor_record_offset":16132,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":16468,"parent_record_offset":16440,"ancestor_record_offset":16132,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":16440,"parent_record_offset":16412,"ancestor_record_offset":16132,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":16244,"parent_record_offset":16196,"ancestor_record_offset":16132,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":16196,"parent_record_offset":16164,"ancestor_record_offset":16132,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":16272,"parent_record_offset":16196,"ancestor_record_offset":16132,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":16300,"parent_record_offset":16196,"ancestor_record_offset":16132,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":16364,"parent_record_offset":16196,"ancestor_record_offset":16132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1983,"record_offset":16328,"parent_record_offset":16196,"ancestor_record_offset":16132,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::device_driver_advice::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 517
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E04
// 首条位置: {"module":1983,"inlinee":77316}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":2128,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":1983,"record_offset":3732,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14525,"source_line":104,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":1983,"record_offset":3760,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":1983,"record_offset":3044,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":4948,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":1983,"record_offset":4980,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":1983,"record_offset":1712,"parent_record_offset":1588,"ancestor_record_offset":1588,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1588,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":1588,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1588,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2236,"parent_record_offset":2200,"ancestor_record_offset":1588,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2268,"parent_record_offset":2236,"ancestor_record_offset":1588,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1983,"record_offset":2048,"parent_record_offset":1992,"ancestor_record_offset":1588,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1983,"record_offset":2200,"parent_record_offset":2164,"ancestor_record_offset":1588,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1588,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1983,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1588,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":1796,"parent_record_offset":1748,"ancestor_record_offset":1588,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1983,"record_offset":1748,"parent_record_offset":1712,"ancestor_record_offset":1588,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1983,"record_offset":3132,"parent_record_offset":3080,"ancestor_record_offset":1588,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":1824,"parent_record_offset":1748,"ancestor_record_offset":1588,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1983,"record_offset":1852,"parent_record_offset":1748,"ancestor_record_offset":1588,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":1916,"parent_record_offset":1748,"ancestor_record_offset":1588,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1983,"record_offset":2344,"parent_record_offset":2316,"ancestor_record_offset":1588,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1983,"record_offset":1880,"parent_record_offset":1748,"ancestor_record_offset":1588,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":1983,"record_offset":2164,"parent_record_offset":2128,"ancestor_record_offset":1588,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":1983,"record_offset":3668,"parent_record_offset":3080,"ancestor_record_offset":1588,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":1983,"record_offset":3168,"parent_record_offset":3132,"ancestor_record_offset":1588,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":1983,"record_offset":3080,"parent_record_offset":3044,"ancestor_record_offset":1588,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/01983.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::render_error_to_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 594
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":35844,"rva":26853344,"va":5395562464}
// 编译实例 RVAs: 0x199bfe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=2
//     target_aliases=101 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26570143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e861926afe","size":5,"soft_fail":false,"target_va":5368995152,"va":5395565290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25930193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e82f5674fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5395562636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=2
//     target_aliases=31 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23329403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e885059cfe","size":5,"soft_fail":false,"target_va":5372235632,"va":5395565030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22812838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e85ae7a3fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395562529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22812975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e8d1e6a3fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395562682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7414273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e801227100","size":5,"soft_fail":false,"target_va":5402978144,"va":5395563866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18032740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e89cd7ecfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395562623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-24258226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e84ed98dfe","size":5,"soft_fail":false,"target_va":5371304880,"va":5395563101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17672488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395562464,"raw_hex":"e8d856f2fe","size":5,"soft_fail":false,"target_va":5377891056,"va":5395563539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36012,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":41364,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":36160,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":37304,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":38452,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3235,"record_offset":36768,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":40176,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":39800,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":3235,"record_offset":36240,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":45350,"source_line":7,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35844,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36092,"parent_record_offset":36068,"ancestor_record_offset":35844,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":35844,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36912,"parent_record_offset":36876,"ancestor_record_offset":35844,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36044,"parent_record_offset":36012,"ancestor_record_offset":35844,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":36488,"parent_record_offset":36432,"ancestor_record_offset":35844,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":3235,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35844,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3235,"record_offset":36876,"parent_record_offset":36840,"ancestor_record_offset":35844,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":36208,"parent_record_offset":36184,"ancestor_record_offset":35844,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":36460,"parent_record_offset":36432,"ancestor_record_offset":35844,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":38528,"parent_record_offset":38496,"ancestor_record_offset":35844,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":36432,"parent_record_offset":36404,"ancestor_record_offset":35844,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":37328,"parent_record_offset":37304,"ancestor_record_offset":35844,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3235,"record_offset":36272,"parent_record_offset":36240,"ancestor_record_offset":35844,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":38496,"parent_record_offset":38452,"ancestor_record_offset":35844,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":37352,"parent_record_offset":37328,"ancestor_record_offset":35844,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3235,"record_offset":36184,"parent_record_offset":36160,"ancestor_record_offset":35844,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":36356,"parent_record_offset":36240,"ancestor_record_offset":35844,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":36116,"parent_record_offset":36092,"ancestor_record_offset":35844,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":38560,"parent_record_offset":38452,"ancestor_record_offset":35844,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":3235,"record_offset":36840,"parent_record_offset":36804,"ancestor_record_offset":35844,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":36328,"parent_record_offset":36240,"ancestor_record_offset":35844,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":3235,"record_offset":36804,"parent_record_offset":36768,"ancestor_record_offset":35844,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":3235,"record_offset":39868,"parent_record_offset":39832,"ancestor_record_offset":35844,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$3::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 631
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18D15
// 首条位置: {"module":3235,"record_offset":41452,"rva":4123808,"va":5372832928}
// 编译实例 RVAs: 0x3eeca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5077099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372832928,"raw_hex":"e86b784d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372832944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$3::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 639
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":41576,"rva":4123952,"va":5372833072}
// 编译实例 RVAs: 0x3eed30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5076955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833072,"raw_hex":"e8db774d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372833088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$4::active
// 记录形式: compiled_procedure; instances=1
// 源码行: 652
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":41700,"rva":26856336,"va":5395565456}
// 编译实例 RVAs: 0x199cb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::localize_graphics_init_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 673
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3235,"record_offset":41824,"rva":26856416,"va":5395565536}
// 编译实例 RVAs: 0x199cbe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22816053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e8cbdaa3fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395565744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22816624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e890d8a3fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395566331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8027418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e81a7d7a00","size":5,"soft_fail":false,"target_va":5403593360,"va":5395565937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7411913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e8c9187100","size":5,"soft_fail":false,"target_va":5402978144,"va":5395566226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18036141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e853caecfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5395566024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-18039098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e8c6beecfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5395566213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-19412718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395565536,"raw_hex":"e812c9d7fe","size":5,"soft_fail":false,"target_va":5376152880,"va":5395565593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":42608,"parent_record_offset":41824,"ancestor_record_offset":41824,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3235,"record_offset":41980,"parent_record_offset":41824,"ancestor_record_offset":41824,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":43228,"parent_record_offset":41824,"ancestor_record_offset":41824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3235,"record_offset":42692,"parent_record_offset":41824,"ancestor_record_offset":41824,"inlinee":9155,"source_line":187,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":42400,"parent_record_offset":42372,"ancestor_record_offset":41824,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":42428,"parent_record_offset":42400,"ancestor_record_offset":41824,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":43372,"parent_record_offset":43348,"ancestor_record_offset":41824,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":42372,"parent_record_offset":42300,"ancestor_record_offset":41824,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3235,"record_offset":43348,"parent_record_offset":43324,"ancestor_record_offset":41824,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3235,"record_offset":42656,"parent_record_offset":42632,"ancestor_record_offset":41824,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":43524,"parent_record_offset":43496,"ancestor_record_offset":41824,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":42300,"parent_record_offset":42264,"ancestor_record_offset":41824,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":42264,"parent_record_offset":42228,"ancestor_record_offset":41824,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":43108,"parent_record_offset":42928,"ancestor_record_offset":41824,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3235,"record_offset":43324,"parent_record_offset":43300,"ancestor_record_offset":41824,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":42180,"parent_record_offset":42140,"ancestor_record_offset":41824,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":42140,"parent_record_offset":42100,"ancestor_record_offset":41824,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":42060,"parent_record_offset":42020,"ancestor_record_offset":41824,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":42100,"parent_record_offset":42060,"ancestor_record_offset":41824,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3235,"record_offset":42020,"parent_record_offset":41980,"ancestor_record_offset":41824,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3235,"record_offset":42228,"parent_record_offset":42180,"ancestor_record_offset":41824,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3235,"record_offset":42548,"parent_record_offset":42520,"ancestor_record_offset":41824,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3235,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":41824,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3235,"record_offset":42520,"parent_record_offset":42180,"ancestor_record_offset":41824,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":43252,"parent_record_offset":43228,"ancestor_record_offset":41824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3235,"record_offset":42464,"parent_record_offset":42428,"ancestor_record_offset":41824,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03235.jsonl.gz"}
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
// 完整结构及原始类型记录: gpu_selection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_gpu_selector::gpu_selection::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner::run_gui", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options::closure_env$1", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_gpu_selector::gpu_selection::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::GpuSelectionReason
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_gpu_selector::gpu_selection::GpuSelectionReason>", "constructor": "enum2$", "arguments": ["tg_gpu_selector::gpu_selection::GpuSelectionReason"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tg_gpu_selector::gpu_selection::GpuSelectionReason> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::GpuSelectionReason>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::HyperlinkStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_gpu_selector::gpu_selection::HyperlinkStyle>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_gpu_selector::gpu_selection::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::pressed<tg_gpu_selector::gpu_selection::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::pressed", "arguments": ["tg_gpu_selector::gpu_selection::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<tg_gpu_selector::gpu_selection::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["tg_gpu_selector::gpu_selection::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::LocalizedError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_gpu_selector::gpu_selection::LocalizedError>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_gpu_selector::gpu_selection::LocalizedError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::Message
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 148
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_gpu_selector::gpu_selection::Message>", "constructor": "enum2$", "arguments": ["tg_gpu_selector::gpu_selection::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "assoc$", "arguments": ["Item", "enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "futures_core::stream::Stream", "arguments": ["assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::pin::Pin", "arguments": ["alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_gpu_selector::gpu_selection::Message> > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::Radio", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::Group", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::Container", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::Scrollable", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::Nested<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::Nested", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::overlay::nested::Nested<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::overlay::nested::Nested<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::user_interface::UserInterface<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::user_interface::UserInterface<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<iced_widget::scrollable::Viewport>", "assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_gpu_selector::gpu_selection::Message> > > >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_runtime::command::Command", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::command::Command<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::command::Command<enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::layout::node::Node,core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::layout::node::Node", "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global>,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global>", "tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "ref_mut$", "arguments": ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "tuple$", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnMut::call_mut<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),tuple$<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ops::function::FnMut::call_mut", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "tuple$<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_core::widget::operation::Operation", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure_env$2", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::build<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_runtime::user_interface::UserInterface::build", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::update<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::update", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::tag<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::tag", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::tag<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::tag", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::tag<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::tag", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$2", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$0", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$1", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::next_to_each_other<iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::next_to_each_other", "arguments": ["iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::flex::resolve<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::layout::flex::resolve", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gpu_selection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gpu_selection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::Controls::new
// 信息 ID: 1551c4d2aca6b0755eb48797
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_gpu_selector::run_gpu_selection_gui; module=269, inline_record=1704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::device_driver_advice
// 信息 ID: dbf824e10cd02da204241d71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0x12fd324 -> 0x1998950 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1301b86 -> 0x1998950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::device_driver_advice::closure$0
// 信息 ID: 2e5c6d5c346c28b3e427fb92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_gpu_selector::gpu_selection::device_driver_advice", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global>,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global>,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=1983, inline_record=1588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$0::program_outcome
// 信息 ID: a4966a8eba44454f84e3a6e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=779, inline_record=14276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::update
// 信息 ID: 3b5d3e446d74aeb7fbbd1593
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > > @ 0x12f941f -> 0x1995960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view
// 信息 ID: ec8560c143b6960a339f84b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::State::new<tg_gpu_selector::gpu_selection::Controls> @ 0x12f3c91 -> 0x1995a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_gpu_selector::gpu_selection::Controls> @ 0x12f3f0c -> 0x1995a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_gpu_selector::gpu_selection::Controls> @ 0x12f4282 -> 0x1995a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$0
// 信息 ID: 827939a56e91296f3f5f521d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; module=1983, inline_record=19228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$1
// 信息 ID: 3f3ffeffb2f04c636b1b598e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=1983, inline_record=43080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$2
// 信息 ID: be77cc749fe116914a689df7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 80d49587b3c5a95afabc153a: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=1983, inline_record=16132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$3::active
// 信息 ID: 391bd338d2830a1db2613a44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<tg_gpu_selector::gpu_selection::HyperlinkStyle>; module=3244, inline_record=424; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<tg_gpu_selector::gpu_selection::HyperlinkStyle>; module=3244, inline_record=628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$3::hovered
// 信息 ID: 568eeac5687aec0dc2b18832
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$4::active
// 信息 ID: 1fc3d872ee366537e1f23a8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$5::fmt
// 信息 ID: e211cbbb8bd6d559b3a474ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$8::clone
// 信息 ID: da217b1a0a6ae1d5440eb154
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::radio::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=3246, inline_record=3516; 不是独立调用边。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=3248, inline_record=14060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::localize_graphics_init_error
// 信息 ID: ce60f13875c7438a8f0956c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1301ba3 -> 0x199cbe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::render_error_to_ui
// 信息 ID: 700daeb36cb7121367f0a099
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1301bab -> 0x199bfe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::impl$2::view @ 0x1996080 -> 0x199bfe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
