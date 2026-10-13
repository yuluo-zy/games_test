// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/hint_binding_renderer.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1eb7b7caf356d729f70d92302180a4a58202c0f4e3367db50df1f3d3cc5361e9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hint_binding_renderer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2644, 3943, 3944, 3981, 3983, 4766, 4776
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4776,"record_offset":560,"rva":45165056,"va":5413874176}
// 编译实例 RVAs: 0x2b12a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-38197476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e81c27b9fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413876175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44332651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e895895bfd","size":5,"soft_fail":false,"target_va":5369542000,"va":5413874646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37674267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8e522c1fd","size":5,"soft_fail":false,"target_va":5376201216,"va":5413875478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e7a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37634860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8d4bcc1fd","size":5,"soft_fail":false,"target_va":5376239520,"va":5413874375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e990
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37634739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e84dbdc1fd","size":5,"soft_fail":false,"target_va":5376240016,"va":5413874750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108a880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27823933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8c37057fe","size":5,"soft_fail":false,"target_va":5386053760,"va":5413877688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428809a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2696117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e84bdcd6ff","size":5,"soft_fail":false,"target_va":5411178912,"va":5413875024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142880ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2695398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e81adfd6ff","size":5,"soft_fail":false,"target_va":5411179424,"va":5413874817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142880f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2694884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e81ce1d6ff","size":5,"soft_fail":false,"target_va":5411180352,"va":5413875231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428815c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2695381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e82bdfd6ff","size":5,"soft_fail":false,"target_va":5411182016,"va":5413877392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142881a90
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2692742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e87ae9d6ff","size":5,"soft_fail":false,"target_va":5411183248,"va":5413875985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142882190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2692298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e836ebd6ff","size":5,"soft_fail":false,"target_va":5411185040,"va":5413877333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428824a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2691481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e867eed6ff","size":5,"soft_fail":false,"target_va":5411185824,"va":5413877300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13d60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e864110000","size":5,"soft_fail":false,"target_va":5413879136,"va":5413874679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8ba120000","size":5,"soft_fail":false,"target_va":5413879504,"va":5413874705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e83c150000","size":5,"soft_fail":false,"target_va":5413879872,"va":5413874431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3181499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8bb8b3000","size":5,"soft_fail":false,"target_va":5417057792,"va":5413876288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-29081908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8cc3e44fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5413876767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2744929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e861e22900","size":5,"soft_fail":false,"target_va":5416622560,"va":5413877626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2744621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e82de12900","size":5,"soft_fail":false,"target_va":5416622240,"va":5413877614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<system_onboarding::hint_binding::BindingHintDisplay> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e833030000","size":5,"soft_fail":false,"target_va":5413878688,"va":5413877864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::keycode_to_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35522005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e82bfae1fd","size":5,"soft_fail":false,"target_va":5378355040,"va":5413877040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::mouse_button_to_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35521637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e89bfbe1fd","size":5,"soft_fail":false,"target_va":5378355088,"va":5413876720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding::HintMouseInput::compact_text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-57144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8c820ffff","size":5,"soft_fail":false,"target_va":5413817200,"va":5413874339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding::HintMouseInput::graphic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-57502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8621fffff","size":5,"soft_fail":false,"target_va":5413817776,"va":5413875273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding_renderer::render_hints
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e821fbffff","size":5,"soft_fail":false,"target_va":5413874176,"va":5413875418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding_renderer::render_hints::fmtinput
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8370b0000","size":5,"soft_fail":false,"target_va":5413878464,"va":5413875588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-37666363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8c541c1fd","size":5,"soft_fail":false,"target_va":5376208368,"va":5413874726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-37689174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8aae8c0fd","size":5,"soft_fail":false,"target_va":5376185232,"va":5413874401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-37689267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e84de8c0fd","size":5,"soft_fail":false,"target_va":5376185152,"va":5413874414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-37689191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e899e8c0fd","size":5,"soft_fail":false,"target_va":5376185296,"va":5413874482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-37689152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413874176,"raw_hex":"e8c0e8c0fd","size":5,"soft_fail":false,"target_va":5376185360,"va":5413874507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4776,"record_offset":3320,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4776,"record_offset":3688,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4776,"record_offset":760,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding_renderer::current_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":1040,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":145338,"source_line":217,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding_renderer::render_hints::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":2016,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":145339,"source_line":64,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding_renderer::render_hints::fmtkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":3748,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":145342,"source_line":55,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4776,"record_offset":1292,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4776,"record_offset":728,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":560,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":560,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":560,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":560,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4776,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":560,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":2192,"parent_record_offset":2164,"ancestor_record_offset":560,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":560,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":560,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4776,"record_offset":1204,"parent_record_offset":1040,"ancestor_record_offset":560,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4776,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":560,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4776,"record_offset":4632,"parent_record_offset":4600,"ancestor_record_offset":560,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4776,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":560,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$248::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":4776,"record_offset":1092,"parent_record_offset":1040,"ancestor_record_offset":560,"inlinee":145341,"source_line":38,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":560,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":560,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":560,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4776,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4776,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":560,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4776,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":560,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4776,"record_offset":1148,"parent_record_offset":1040,"ancestor_record_offset":560,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::fmtkey::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":560,"inlinee":145344,"source_line":56,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4776,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":560,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4776,"record_offset":1372,"parent_record_offset":1316,"ancestor_record_offset":560,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4776,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":560,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::fmtkey
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237BE
// 首条位置: {"module":4776,"inlinee":145342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3748,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::fmtkey::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3748,"inlinee":145344,"source_line":56,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::fmtkey::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237C0
// 首条位置: {"module":4776,"inlinee":145344}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3796,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::fmtinput
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4776,"record_offset":7060,"rva":45169344,"va":5413878464}
// 编译实例 RVAs: 0x2b13ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-38199822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413878464,"raw_hex":"e8f21db9fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413878505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-38199899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413878464,"raw_hex":"e8a51db9fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413878598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45156303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413878464,"raw_hex":"e831f84efd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413878490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2742167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413878464,"raw_hex":"e897d72900","size":5,"soft_fail":false,"target_va":5416620794,"va":5413878622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::InputBinding::format_to_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35523974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413878464,"raw_hex":"e87af2e1fd","size":5,"soft_fail":false,"target_va":5378354608,"va":5413878577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4776,"record_offset":7212,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4776,"record_offset":7672,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7492,"parent_record_offset":7468,"ancestor_record_offset":7060,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":7060,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":7060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":7060,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":7060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7468,"parent_record_offset":7440,"ancestor_record_offset":7060,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4776,"record_offset":7848,"parent_record_offset":7824,"ancestor_record_offset":7060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":7384,"parent_record_offset":7356,"ancestor_record_offset":7060,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":7060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":7440,"parent_record_offset":7412,"ancestor_record_offset":7060,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":7412,"parent_record_offset":7384,"ancestor_record_offset":7060,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4776,"record_offset":7800,"parent_record_offset":7780,"ancestor_record_offset":7060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4776,"record_offset":7268,"parent_record_offset":7240,"ancestor_record_offset":7060,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4776,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":7060,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4776,"record_offset":7324,"parent_record_offset":7296,"ancestor_record_offset":7060,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4776,"record_offset":7240,"parent_record_offset":7212,"ancestor_record_offset":7060,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4776,"record_offset":7356,"parent_record_offset":7324,"ancestor_record_offset":7060,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4776,"record_offset":7596,"parent_record_offset":7324,"ancestor_record_offset":7060,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4776,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":7060,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4776,"record_offset":7740,"parent_record_offset":7672,"ancestor_record_offset":7060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::fmtinput::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4776,"record_offset":7716,"parent_record_offset":7672,"ancestor_record_offset":7060,"inlinee":145346,"source_line":60,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::fmtinput::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237C2
// 首条位置: {"module":4776,"inlinee":145346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237BB
// 首条位置: {"module":4776,"inlinee":145339}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E45
// 首条位置: {"module":3981,"inlinee":134725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3981,"record_offset":18100,"parent_record_offset":18068,"ancestor_record_offset":18068,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E42
// 首条位置: {"module":3981,"inlinee":134722}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3981,"record_offset":16544,"parent_record_offset":16504,"ancestor_record_offset":16504,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3981,"record_offset":7664,"rva":42468080,"va":5411177200}
// 编译实例 RVAs: 0x28802f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41635311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e811b284fd","size":5,"soft_fail":false,"target_va":5369542000,"va":5411177306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142882200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e8ef1c0000","size":5,"soft_fail":false,"target_va":5411185152,"va":5411177740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13d60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2701798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e8e6392900","size":5,"soft_fail":false,"target_va":5413879136,"va":5411177333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e8423b2900","size":5,"soft_fail":false,"target_va":5413879504,"va":5411177353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e8dd3b2900","size":5,"soft_fail":false,"target_va":5413879872,"va":5411177566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding_renderer::render_hints
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2696760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e838262900","size":5,"soft_fail":false,"target_va":5413874176,"va":5411177411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34969009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e84f6aeafd","size":5,"soft_fail":false,"target_va":5376208368,"va":5411177372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34992408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411177200,"raw_hex":"e8e80eeafd","size":5,"soft_fail":false,"target_va":5376185152,"va":5411177555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3981,"record_offset":7828,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7980,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":8092,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":10274,"source_line":1421,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3981,"record_offset":8148,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":7944,"parent_record_offset":7864,"ancestor_record_offset":7664,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":7908,"parent_record_offset":7864,"ancestor_record_offset":7664,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3981,"record_offset":7864,"parent_record_offset":7828,"ancestor_record_offset":7664,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3981,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":7664,"inlinee":134714,"source_line":182,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":7664,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":8056,"parent_record_offset":8004,"ancestor_record_offset":7664,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":7664,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E3A
// 首条位置: {"module":3981,"inlinee":134714}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E3E
// 首条位置: {"module":3981,"inlinee":134718}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3981,"record_offset":12664,"parent_record_offset":12628,"ancestor_record_offset":12628,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":12804,"parent_record_offset":12628,"ancestor_record_offset":12628,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":12924,"parent_record_offset":12628,"ancestor_record_offset":12628,"inlinee":10274,"source_line":1421,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":12768,"parent_record_offset":12700,"ancestor_record_offset":12628,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":12736,"parent_record_offset":12700,"ancestor_record_offset":12628,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3981,"record_offset":12700,"parent_record_offset":12664,"ancestor_record_offset":12628,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::closure$3::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3981,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":12628,"inlinee":134720,"source_line":193,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":12628,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":12888,"parent_record_offset":12832,"ancestor_record_offset":12628,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":12832,"parent_record_offset":12804,"ancestor_record_offset":12628,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E40
// 首条位置: {"module":3981,"inlinee":134720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3981,"record_offset":6976,"rva":42467504,"va":5411176624}
// 编译实例 RVAs: 0x28800b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41634735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e851b484fd","size":5,"soft_fail":false,"target_va":5369542000,"va":5411176730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142882200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e82f1f0000","size":5,"soft_fail":false,"target_va":5411185152,"va":5411177164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13d60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e8263c2900","size":5,"soft_fail":false,"target_va":5413879136,"va":5411176757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e8823d2900","size":5,"soft_fail":false,"target_va":5413879504,"va":5411176777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e81d3e2900","size":5,"soft_fail":false,"target_va":5413879872,"va":5411176990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding_renderer::render_hints
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2697336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e878282900","size":5,"soft_fail":false,"target_va":5413874176,"va":5411176835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34968433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e88f6ceafd","size":5,"soft_fail":false,"target_va":5376208368,"va":5411176796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176624,"raw_hex":"e82811eafd","size":5,"soft_fail":false,"target_va":5376185152,"va":5411176979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3981,"record_offset":7140,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7292,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7404,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":10274,"source_line":1421,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3981,"record_offset":7460,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":7256,"parent_record_offset":7176,"ancestor_record_offset":6976,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":7220,"parent_record_offset":7176,"ancestor_record_offset":6976,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3981,"record_offset":7176,"parent_record_offset":7140,"ancestor_record_offset":6976,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::render_hints::closure$4::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3981,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":6976,"inlinee":134712,"source_line":208,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":6976,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7368,"parent_record_offset":7316,"ancestor_record_offset":6976,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":6976,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::render_hints::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E38
// 首条位置: {"module":3981,"inlinee":134712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::current_texture
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x237BA
// 首条位置: {"module":4776,"inlinee":145338}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4776,"record_offset":1204,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$248::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":4776,"record_offset":1092,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":145341,"source_line":38,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4776,"record_offset":1148,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4776,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":1040,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4776,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":1040,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4776,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":1040,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4776,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":1040,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/04776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding::BindingHintDisplay::prepare_for_display
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D72
// 首条位置: {"module":3944,"inlinee":134514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::cleanup
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3944,"record_offset":6444,"parent_record_offset":6304,"ancestor_record_offset":6304,"inlinee":134517,"source_line":322,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::filter_to_mode
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":6304,"inlinee":134515,"source_line":197,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::resolve_steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3944,"record_offset":6380,"parent_record_offset":6304,"ancestor_record_offset":6304,"inlinee":134516,"source_line":244,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3944,"record_offset":6412,"parent_record_offset":6380,"ancestor_record_offset":6304,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding::BindingHintDisplay::resolve_steam
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D74
// 首条位置: {"module":3944,"inlinee":134516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3944,"record_offset":6412,"parent_record_offset":6380,"ancestor_record_offset":6380,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding::BindingHintDisplay::resolve_steam_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4766,"record_offset":9692,"rva":45111808,"va":5413820928}
// 编译实例 RVAs: 0x2b05a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-38142645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e84bfdb9fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413821344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141088e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27775688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8382d58fe","size":5,"soft_fail":false,"target_va":5386047088,"va":5413822771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108ac40
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27766832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8d04f58fe","size":5,"soft_fail":false,"target_va":5386054720,"va":5413821547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108b4f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27765856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8a05358fe","size":5,"soft_fail":false,"target_va":5386056944,"va":5413822795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420bc0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10788886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8ea5f5bff","size":5,"soft_fail":false,"target_va":5403033792,"va":5413822673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b05a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8fbfdffff","size":5,"soft_fail":false,"target_va":5413820928,"va":5413821440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b06730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e80b000000","size":5,"soft_fail":false,"target_va":5413824304,"va":5413824288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2798749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e89db42a00","size":5,"soft_fail":false,"target_va":5416621600,"va":5413822846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::steam::SteamInput::get_action_origin_glyphs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26575006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413820928,"raw_hex":"e8627f6afe","size":5,"soft_fail":false,"target_va":5387246160,"va":5413821161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":18284,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":9880,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4766,"record_offset":18484,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4766,"record_offset":18340,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4766,"record_offset":13228,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4766,"record_offset":18572,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":10028,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4766,"record_offset":9992,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4766,"record_offset":13172,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":9692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":9692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":9692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":10364,"parent_record_offset":10272,"ancestor_record_offset":9692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":9692,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":9692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":11708,"parent_record_offset":11684,"ancestor_record_offset":9692,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":9692,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":9692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":9692,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":18364,"parent_record_offset":18340,"ancestor_record_offset":9692,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":11612,"parent_record_offset":11584,"ancestor_record_offset":9692,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":4766,"record_offset":18388,"parent_record_offset":18364,"ancestor_record_offset":9692,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4766,"record_offset":10332,"parent_record_offset":10300,"ancestor_record_offset":9692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=59
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":9692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4766,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9692,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4766,"record_offset":18512,"parent_record_offset":18484,"ancestor_record_offset":9692,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4766,"record_offset":13196,"parent_record_offset":13172,"ancestor_record_offset":9692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":18536,"parent_record_offset":18512,"ancestor_record_offset":9692,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4766,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9692,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4766,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":9692,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D6C
// 首条位置: {"module":3943,"inlinee":134508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3943,"record_offset":1488,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3943,"record_offset":1884,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3943,"record_offset":1572,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3943,"record_offset":1404,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":21764,"source_line":2480,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3943,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1364,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3943,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1364,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3943,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1364,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3943,"record_offset":1748,"parent_record_offset":1692,"ancestor_record_offset":1364,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3943,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":1364,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3943,"record_offset":1720,"parent_record_offset":1692,"ancestor_record_offset":1364,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3943,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1364,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3943,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1364,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3943,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":1364,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3943,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":1364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3943,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":1364,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3943,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":1364,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/03943.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding::BindingHintDisplay::cleanup
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D75
// 首条位置: {"module":3944,"inlinee":134517}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4766,"record_offset":5152,"rva":45109600,"va":5413818720}
// 编译实例 RVAs: 0x2b05160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-38140699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8e504bafd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413819382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-38140929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8ff03bafd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413819628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45097180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e824df4ffd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413819367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108a880
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27765238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e80a5658fe","size":5,"soft_fail":false,"target_va":5386053760,"va":5413818993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108ac40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27764816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8b05758fe","size":5,"soft_fail":false,"target_va":5386054720,"va":5413819531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108b4f0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27762292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e88c6158fe","size":5,"soft_fail":false,"target_va":5386056944,"va":5413819231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420bbc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10787016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e838675bff","size":5,"soft_fail":false,"target_va":5403032704,"va":5413819715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b05160
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e884ffffff","size":5,"soft_fail":false,"target_va":5413818720,"va":5413818839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b06730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8830d0000","size":5,"soft_fail":false,"target_va":5413824304,"va":5413820840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: hint_binding_renderer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3236819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8d3633100","size":5,"soft_fail":false,"target_va":5417057072,"va":5413820248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2800577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8c1bb2a00","size":5,"soft_fail":false,"target_va":5416620769,"va":5413820187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2801393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413818720,"raw_hex":"e8f1be2a00","size":5,"soft_fail":false,"target_va":5416621600,"va":5413820202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4766,"record_offset":6348,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":8836,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5432,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4766,"record_offset":6316,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4766,"record_offset":8988,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":6012,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4766,"record_offset":6044,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4766,"record_offset":5336,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::is_some
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":4766,"record_offset":6228,"parent_record_offset":5152,"ancestor_record_offset":5152,"inlinee":108173,"source_line":193,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":5152,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":5152,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":5152,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":5152,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":5152,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":5152,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6424,"parent_record_offset":6384,"ancestor_record_offset":5152,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4766,"record_offset":6812,"parent_record_offset":6756,"ancestor_record_offset":5152,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4766,"record_offset":6384,"parent_record_offset":6348,"ancestor_record_offset":5152,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":7324,"parent_record_offset":7300,"ancestor_record_offset":5152,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":5152,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":5152,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":5152,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":5152,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4766,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":5152,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":5152,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":8864,"parent_record_offset":8836,"ancestor_record_offset":5152,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5572,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5468,"parent_record_offset":5432,"ancestor_record_offset":5152,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5860,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5664,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":5152,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4766,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":5152,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4766,"record_offset":5600,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4766,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":5152,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4766,"record_offset":7080,"parent_record_offset":7048,"ancestor_record_offset":5152,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4766,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":5152,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":5940,"parent_record_offset":5912,"ancestor_record_offset":5152,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":5792,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4766,"record_offset":5888,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4766,"record_offset":5828,"parent_record_offset":5468,"ancestor_record_offset":5152,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4766,"record_offset":5400,"parent_record_offset":5336,"ancestor_record_offset":5152,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4766,"record_offset":5368,"parent_record_offset":5336,"ancestor_record_offset":5152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4766,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5152,"inlinee":145299,"source_line":331,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":4766,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":5152,"inlinee":108009,"source_line":344,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x23793
// 首条位置: {"module":4766,"inlinee":145299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4766,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":6068,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 344
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A5E9
// 首条位置: {"module":2644,"inlinee":108009}
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
// 完整结构及原始类型记录: hint_binding_renderer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_onboarding::hint_binding_renderer::HintRenderMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hint_binding_renderer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hint_binding_renderer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding::BindingHintDisplay::cleanup
// 信息 ID: c9c1c2ebe751328a57d0770e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=6444; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_popup::hint_draw::closure$2; module=3981, inline_record=2124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl
// 信息 ID: df7e19713f8eb1ad359b2a04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > > @ 0x28702e3 -> 0x2b05160 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_popup::hint_draw::closure$2 @ 0x287f7c6 -> 0x2b05160 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl @ 0x2b051d7 -> 0x2b05160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding::BindingHintDisplay::prepare_for_display
// 信息 ID: cdf211db2fe4e0adc1b8bb1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=6304; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_popup::hint_draw::closure$2; module=3981, inline_record=1984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding::BindingHintDisplay::resolve_steam
// 信息 ID: b3e5d68609de8cad3c233862
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=6380; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_popup::hint_draw::closure$2; module=3981, inline_record=2060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding::BindingHintDisplay::resolve_steam_impl
// 信息 ID: 07e3f6d78e735ad7d788a99b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > > @ 0x28702ab -> 0x2b05a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_popup::hint_draw::closure$2 @ 0x287f7a1 -> 0x2b05a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::current_texture
// 信息 ID: e7a558fe81355a07464f2bf4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=1040; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=1640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$0
// 信息 ID: 1b96e0e1969aa09ea9d56a02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_binding_renderer::impl$1::cleanup_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl; module=4766, inline_record=6068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::impl$1::cleanup_impl::closure$1
// 信息 ID: 403d86a9e1a9c3a7f6e1b3f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_onboarding::hint_binding_renderer::impl$1::cleanup_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl; module=4766, inline_record=5688; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_binding::BindingHintDisplay::cleanup_impl; module=4766, inline_record=5752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure$1
// 信息 ID: 02cb22198f746d3c4a34e237
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,enum2$<system_onboarding::hint_binding::BindingHintDisplay>,tuple$<>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,enum2$<system_onboarding::hint_binding::BindingHintDisplay>,tuple$<>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,enum2$<system_onboarding::hint_binding::BindingHintDisplay>,tuple$<>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_binding_renderer::impl$1::resolve_steam_impl::closure_env$1> > > > >; module=3943, inline_record=1364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints
// 信息 ID: f10a5bbdb2e06b2f1a9376eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_popup::hint_draw::closure$2 @ 0x287f7f7 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_always_on::hint_draw_always_on::closure$3 @ 0x287fb74 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding_renderer::render_hints::closure$4 @ 0x2880183 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding_renderer::render_hints::closure$4 @ 0x2880296 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding_renderer::render_hints::closure$2 @ 0x28803c3 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding_renderer::render_hints::closure$2 @ 0x28804d6 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> > @ 0x2880d0f -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> > @ 0x2880f05 -> 0x2b12a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$0
// 信息 ID: 875c6367431962b459e6871e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=2016; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=3860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$0::closure$0
// 信息 ID: 39255da8a48e57429f92b0ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::render_hints::closure$0::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure$0::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure$0::closure_env$0,ref$<str$> >; module=3981, inline_record=17628; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure$0::closure_env$0,ref$<str$> >; module=3981, inline_record=18032; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure$0::closure_env$0,ref$<str$> >; module=3981, inline_record=18068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$1
// 信息 ID: 9bbb5165fddfe1751d6443bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::render_hints::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$1,ref$<str$> >; module=3981, inline_record=16504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$2
// 信息 ID: 58c8badbc78f5c579addc129
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::render_hints::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$2,ref$<str$> > @ 0x2880b7c -> 0x28802f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$2::closure$0
// 信息 ID: ac3e0fa22e660aa4648823f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints::closure$2; module=3981, inline_record=8116; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints::closure$2; module=3981, inline_record=8316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$3
// 信息 ID: a78ffebf67fc7e362ddd012e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::render_hints::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> >; module=3981, inline_record=12628; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> >; module=3981, inline_record=13352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$3::closure$0
// 信息 ID: 99038f83e4e5295f7a54d2f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> >; module=3981, inline_record=12948; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$3,ref$<str$> >; module=3981, inline_record=13672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$4
// 信息 ID: af5311122787b08c62b483e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_binding_renderer::render_hints::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_binding_renderer::render_hints::closure_env$4,ref$<str$> > @ 0x288111c -> 0x28800b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::closure$4::closure$0
// 信息 ID: ce6901bed4434947a3a0b349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints::closure$4; module=3981, inline_record=7428; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints::closure$4; module=3981, inline_record=7628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::fmtinput
// 信息 ID: aa0fb59118bf2a995d098b30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::fmtinput::closure$0
// 信息 ID: 097305291b7e9d65a0548707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints::fmtinput", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints::fmtinput; module=4776, inline_record=7716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::fmtkey
// 信息 ID: 01ef22f87fa9542e453451c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=3748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_binding_renderer::render_hints::fmtkey::closure$0
// 信息 ID: 2d7855b618704177826196cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_binding_renderer::render_hints::fmtkey", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_binding_renderer::render_hints; module=4776, inline_record=3796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
