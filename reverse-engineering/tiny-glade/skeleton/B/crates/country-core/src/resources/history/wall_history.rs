// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/wall_history.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e5301f93ebcd262f797d5c37f03e47a06bf77198f8300f22dce236263a9195d5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_history.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 555, 565
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":10576,"rva":11023104,"va":5379732224}
// 编译实例 RVAs: 0xa83300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e812e6b4ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5379735257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4651225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e82707b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379732708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4652829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8e300b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379734328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087e0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2120706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8fea3dfff","size":5,"soft_fail":false,"target_va":5377614016,"va":5379734717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408974b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2017122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e89e38e1ff","size":5,"soft_fail":false,"target_va":5377717424,"va":5379734541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a82940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e85bebffff","size":5,"soft_fail":false,"target_va":5379729728,"va":5379735008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10368996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8e4379e00","size":5,"soft_fail":false,"target_va":5390101424,"va":5379732423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141696f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12662391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e87736c100","size":5,"soft_fail":false,"target_va":5392396112,"va":5379733716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32114400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8e006ea01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379732763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31806907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8bb55e501","size":5,"soft_fail":false,"target_va":5411540992,"va":5379734080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31806935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8d755e501","size":5,"soft_fail":false,"target_va":5411540963,"va":5379734023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31808213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8d55ae501","size":5,"soft_fail":false,"target_va":5411542064,"va":5379733846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31807942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8c659e501","size":5,"soft_fail":false,"target_va":5411541904,"va":5379733957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379732224,"raw_hex":"e8a6150000","size":5,"soft_fail":false,"target_va":5379739328,"va":5379733781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":12560,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":555,"record_offset":13796,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":13128,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":555,"record_offset":12864,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":555,"record_offset":10788,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":555,"record_offset":13628,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":27915,"source_line":28,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":555,"record_offset":11024,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":555,"record_offset":10760,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":555,"record_offset":10816,"parent_record_offset":10576,"ancestor_record_offset":10576,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":10576,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":10576,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":10576,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":12940,"parent_record_offset":12904,"ancestor_record_offset":10576,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":11556,"parent_record_offset":11380,"ancestor_record_offset":10576,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":12480,"parent_record_offset":12392,"ancestor_record_offset":10576,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":555,"record_offset":12904,"parent_record_offset":12864,"ancestor_record_offset":10576,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":11316,"parent_record_offset":11284,"ancestor_record_offset":10576,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":12420,"parent_record_offset":12392,"ancestor_record_offset":10576,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":10576,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":11380,"parent_record_offset":11348,"ancestor_record_offset":10576,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":11348,"parent_record_offset":11316,"ancestor_record_offset":10576,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":10576,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":11416,"parent_record_offset":11380,"ancestor_record_offset":10576,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":11248,"parent_record_offset":11216,"ancestor_record_offset":10576,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":10576,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":11216,"parent_record_offset":11148,"ancestor_record_offset":10576,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":11188,"parent_record_offset":11148,"ancestor_record_offset":10576,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":10576,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":11284,"parent_record_offset":11248,"ancestor_record_offset":10576,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":11116,"parent_record_offset":11064,"ancestor_record_offset":10576,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":13652,"parent_record_offset":13628,"ancestor_record_offset":10576,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":11148,"parent_record_offset":11116,"ancestor_record_offset":10576,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":555,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10576,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":555,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10576,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10576,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":10576,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":555,"record_offset":10964,"parent_record_offset":10816,"ancestor_record_offset":10576,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":555,"record_offset":10992,"parent_record_offset":10816,"ancestor_record_offset":10576,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":555,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":10576,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":555,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":10576,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":13764,"parent_record_offset":13628,"ancestor_record_offset":10576,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":13708,"parent_record_offset":13628,"ancestor_record_offset":10576,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":555,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":10576,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":555,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":10576,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":555,"record_offset":12448,"parent_record_offset":12420,"ancestor_record_offset":10576,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":555,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10576,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":555,"record_offset":10856,"parent_record_offset":10816,"ancestor_record_offset":10576,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":555,"record_offset":11656,"parent_record_offset":11248,"ancestor_record_offset":10576,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":12320,"parent_record_offset":11064,"ancestor_record_offset":10576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":13676,"parent_record_offset":13652,"ancestor_record_offset":10576,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":10576,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":555,"record_offset":11064,"parent_record_offset":11024,"ancestor_record_offset":10576,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C04
// 首条位置: {"module":555,"record_offset":14052,"rva":11026304,"va":5379735424}
// 编译实例 RVAs: 0xa83f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a83fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735424,"raw_hex":"e81a000000","size":5,"soft_fail":false,"target_va":5379735472,"va":5379735441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735424,"raw_hex":"e8180f0000","size":5,"soft_fail":false,"target_va":5379739328,"va":5379735459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::EditType::new_height_changed
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":14204,"rva":11026352,"va":5379735472}
// 编译实例 RVAs: 0xa83fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4654064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e810fcb8ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379735547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4654423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e8a9fab8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379735922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408974b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2018569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e8f732e1ff","size":5,"soft_fail":false,"target_va":5377717424,"va":5379735988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141696f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12660360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e8882ec100","size":5,"soft_fail":false,"target_va":5392396112,"va":5379735747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_history.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32111474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e872fbe901","size":5,"soft_fail":false,"target_va":5411847168,"va":5379735689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31805179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735472,"raw_hex":"e8fb4ee501","size":5,"soft_fail":false,"target_va":5411540992,"va":5379735808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":14364,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":555,"record_offset":15084,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":555,"record_offset":14660,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":555,"record_offset":14916,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":27915,"source_line":28,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":14204,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":14204,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14204,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":14728,"parent_record_offset":14692,"ancestor_record_offset":14204,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":14396,"parent_record_offset":14364,"ancestor_record_offset":14204,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":14204,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":555,"record_offset":14692,"parent_record_offset":14660,"ancestor_record_offset":14204,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":14204,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":555,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":14204,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":15052,"parent_record_offset":14916,"ancestor_record_offset":14204,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":14996,"parent_record_offset":14916,"ancestor_record_offset":14204,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":14880,"parent_record_offset":14692,"ancestor_record_offset":14204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":555,"record_offset":14964,"parent_record_offset":14940,"ancestor_record_offset":14204,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":14204,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00555.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::history::EditType::new_height_changed
// 信息 ID: a5fd8713a8852dec66f0c4a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::deferred_wall_history::history_record_merge_walls @ 0x8dd350 -> 0xa83fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::wall_history::impl$0::ev_height_changed @ 0xa83f91 -> 0xa83fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height @ 0xac7d6d -> 0xa83fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_height_changed
// 信息 ID: 0b9785d9761c1f5d55ce9888
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height; module=565, inline_record=60696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_history::impl$0::ev_new_wall
// 信息 ID: e04c98a99aa804e6307a40e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x97f134 -> 0xa83300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa80bbc -> 0xa83300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0xa9448f -> 0xa83300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
