// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/gui/confirm_incoming_panic_repro.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d3595065acd8444248017237ff939399355494bf6400303afb64ab4fb85c4288
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: confirm_incoming_panic_repro.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 38
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.10.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":38,"record_offset":78592,"rva":971392,"va":5369680512}
// 编译实例 RVAs: 0xed280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5400783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e8cf685200","size":5,"soft_fail":false,"target_va":5375081488,"va":5369680700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5400420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e864675200","size":5,"soft_fail":false,"target_va":5375081504,"va":5369681079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8765107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e8b3be8500","size":5,"soft_fail":false,"target_va":5378445680,"va":5369680568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42166430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e89e688302","size":5,"soft_fail":false,"target_va":5411847168,"va":5369680733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13159527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e867ccc800","size":5,"soft_fail":false,"target_va":5382840208,"va":5369680676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41859578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e8fab97e02","size":5,"soft_fail":false,"target_va":5411540963,"va":5369681380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputStream::frame_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8764537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e879bc8500","size":5,"soft_fail":false,"target_va":5378445696,"va":5369681154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_buttons
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11088600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e8d832a900","size":5,"soft_fail":false,"target_va":5380769632,"va":5369681027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_description
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11088365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e8ed31a900","size":5,"soft_fail":false,"target_va":5380769376,"va":5369681006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_title
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11088132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e80431a900","size":5,"soft_fail":false,"target_va":5380769120,"va":5369680983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::show
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11088799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369680512,"raw_hex":"e89f33a900","size":5,"soft_fail":false,"target_va":5380769840,"va":5369681036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":38,"record_offset":78776,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":38,"record_offset":78856,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":38,"record_offset":79540,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":38,"record_offset":79428,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":38,"record_offset":79596,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":38,"record_offset":79456,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":8481,"source_line":27,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::set_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":38,"record_offset":79512,"parent_record_offset":78592,"ancestor_record_offset":78592,"inlinee":8484,"source_line":35,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":38,"record_offset":79188,"parent_record_offset":79164,"ancestor_record_offset":78592,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":38,"record_offset":79212,"parent_record_offset":79188,"ancestor_record_offset":78592,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":38,"record_offset":79772,"parent_record_offset":79748,"ancestor_record_offset":78592,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":38,"record_offset":79164,"parent_record_offset":79136,"ancestor_record_offset":78592,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":38,"record_offset":79748,"parent_record_offset":79692,"ancestor_record_offset":78592,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":38,"record_offset":78824,"parent_record_offset":78800,"ancestor_record_offset":78592,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79076,"parent_record_offset":79048,"ancestor_record_offset":78592,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79720,"parent_record_offset":79692,"ancestor_record_offset":78592,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79692,"parent_record_offset":79668,"ancestor_record_offset":78592,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79136,"parent_record_offset":79104,"ancestor_record_offset":78592,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79104,"parent_record_offset":79076,"ancestor_record_offset":78592,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":38,"record_offset":79668,"parent_record_offset":79644,"ancestor_record_offset":78592,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":38,"record_offset":79012,"parent_record_offset":78984,"ancestor_record_offset":78592,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":38,"record_offset":78984,"parent_record_offset":78956,"ancestor_record_offset":78592,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":38,"record_offset":78928,"parent_record_offset":78884,"ancestor_record_offset":78592,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":38,"record_offset":78956,"parent_record_offset":78928,"ancestor_record_offset":78592,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":38,"record_offset":78884,"parent_record_offset":78856,"ancestor_record_offset":78592,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":79048,"parent_record_offset":79012,"ancestor_record_offset":78592,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":38,"record_offset":79564,"parent_record_offset":79540,"ancestor_record_offset":78592,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":38,"record_offset":79288,"parent_record_offset":79264,"ancestor_record_offset":78592,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":38,"record_offset":78800,"parent_record_offset":78776,"ancestor_record_offset":78592,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":38,"record_offset":79264,"parent_record_offset":79012,"ancestor_record_offset":78592,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":38,"record_offset":79620,"parent_record_offset":79596,"ancestor_record_offset":78592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] rfd::message_dialog::impl$5::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":38,"record_offset":79480,"parent_record_offset":79456,"ancestor_record_offset":78592,"inlinee":8483,"source_line":13,"archive":"_pdb/B/modules/00038.jsonl.gz"}
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
// 完整结构及原始类型记录: confirm_incoming_panic_repro.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: confirm_incoming_panic_repro.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: confirm_incoming_panic_repro.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro
// 信息 ID: 01e64c1acb7411c8832a76e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 287a82fd992ad3fe6973b048: void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)> > @ 0x151c2f -> 0xed280 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)> > > @ 0x434151 -> 0xed280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
