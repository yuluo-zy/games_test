// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 969b95a7b38284e83ba8b9527ec280c86553252ca015693b25050b7a4e76467c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: adjust_wall_ui_mode.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 479, 513, 584, 1585, 1610
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":75840,"rva":9792720,"va":5378501840}
// 编译实例 RVAs: 0x956cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3420526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e892cecbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378502009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3421449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e8f7cacbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378502948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3176623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e85187cfff","size":5,"soft_fail":false,"target_va":5375326672,"va":5378503290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11622327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e8b757b100","size":5,"soft_fail":false,"target_va":5390125184,"va":5378502852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33344343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e857cbfc01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378502820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33037983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e89f1ef801","size":5,"soft_fail":false,"target_va":5411540992,"va":5378503004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4338224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e830324200","size":5,"soft_fail":false,"target_va":5382840208,"va":5378501979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33037969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378501840,"raw_hex":"e8911ef801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378502989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":76052,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":76132,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":513,"record_offset":78988,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":513,"record_offset":78876,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":80376,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":513,"record_offset":78788,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":26164,"source_line":259,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":76348,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":513,"record_offset":80348,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":513,"record_offset":77248,"parent_record_offset":75840,"ancestor_record_offset":75840,"inlinee":26478,"source_line":165,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":76244,"parent_record_offset":76220,"ancestor_record_offset":75840,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":76268,"parent_record_offset":76244,"ancestor_record_offset":75840,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":79440,"parent_record_offset":79416,"ancestor_record_offset":75840,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":76188,"parent_record_offset":76160,"ancestor_record_offset":75840,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":76220,"parent_record_offset":76188,"ancestor_record_offset":75840,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":79416,"parent_record_offset":79360,"ancestor_record_offset":75840,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":513,"record_offset":76160,"parent_record_offset":76132,"ancestor_record_offset":75840,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":76100,"parent_record_offset":76076,"ancestor_record_offset":75840,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":76672,"parent_record_offset":76644,"ancestor_record_offset":75840,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":79388,"parent_record_offset":79360,"ancestor_record_offset":75840,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":79360,"parent_record_offset":79336,"ancestor_record_offset":75840,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":76732,"parent_record_offset":76700,"ancestor_record_offset":75840,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":76700,"parent_record_offset":76672,"ancestor_record_offset":75840,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":79336,"parent_record_offset":79312,"ancestor_record_offset":75840,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":76768,"parent_record_offset":76732,"ancestor_record_offset":75840,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":76604,"parent_record_offset":76568,"ancestor_record_offset":75840,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":76568,"parent_record_offset":76532,"ancestor_record_offset":75840,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":76496,"parent_record_offset":76460,"ancestor_record_offset":75840,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":76532,"parent_record_offset":76496,"ancestor_record_offset":75840,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":76460,"parent_record_offset":76424,"ancestor_record_offset":75840,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":76424,"parent_record_offset":76388,"ancestor_record_offset":75840,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":76388,"parent_record_offset":76348,"ancestor_record_offset":75840,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":77116,"parent_record_offset":77092,"ancestor_record_offset":75840,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":77356,"parent_record_offset":77332,"ancestor_record_offset":75840,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":78144,"parent_record_offset":78116,"ancestor_record_offset":75840,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":77664,"parent_record_offset":77616,"ancestor_record_offset":75840,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":76644,"parent_record_offset":76604,"ancestor_record_offset":75840,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":77544,"parent_record_offset":77508,"ancestor_record_offset":75840,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":75840,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":78544,"parent_record_offset":78512,"ancestor_record_offset":75840,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":77332,"parent_record_offset":77308,"ancestor_record_offset":75840,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":513,"record_offset":78620,"parent_record_offset":78544,"ancestor_record_offset":75840,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":513,"record_offset":78116,"parent_record_offset":77616,"ancestor_record_offset":75840,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":513,"record_offset":77580,"parent_record_offset":77544,"ancestor_record_offset":75840,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":513,"record_offset":77616,"parent_record_offset":77580,"ancestor_record_offset":75840,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":76792,"parent_record_offset":76768,"ancestor_record_offset":75840,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":76816,"parent_record_offset":76792,"ancestor_record_offset":75840,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":513,"record_offset":77464,"parent_record_offset":77440,"ancestor_record_offset":75840,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":513,"record_offset":77440,"parent_record_offset":77416,"ancestor_record_offset":75840,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":77032,"parent_record_offset":77008,"ancestor_record_offset":75840,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":76864,"parent_record_offset":76840,"ancestor_record_offset":75840,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":513,"record_offset":78200,"parent_record_offset":78172,"ancestor_record_offset":75840,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":513,"record_offset":78368,"parent_record_offset":78256,"ancestor_record_offset":75840,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":77392,"parent_record_offset":77284,"ancestor_record_offset":75840,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":78228,"parent_record_offset":78200,"ancestor_record_offset":75840,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":78256,"parent_record_offset":78228,"ancestor_record_offset":75840,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":78660,"parent_record_offset":78256,"ancestor_record_offset":75840,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":77416,"parent_record_offset":77392,"ancestor_record_offset":75840,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":513,"record_offset":78328,"parent_record_offset":78296,"ancestor_record_offset":75840,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":513,"record_offset":78296,"parent_record_offset":78256,"ancestor_record_offset":75840,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":77508,"parent_record_offset":77248,"ancestor_record_offset":75840,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":78172,"parent_record_offset":78144,"ancestor_record_offset":75840,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":78512,"parent_record_offset":78368,"ancestor_record_offset":75840,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":77284,"parent_record_offset":77248,"ancestor_record_offset":75840,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":76840,"parent_record_offset":76816,"ancestor_record_offset":75840,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":77940,"parent_record_offset":77912,"ancestor_record_offset":75840,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":78468,"parent_record_offset":78436,"ancestor_record_offset":75840,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":513,"record_offset":78436,"parent_record_offset":78404,"ancestor_record_offset":75840,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":513,"record_offset":76076,"parent_record_offset":76052,"ancestor_record_offset":75840,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":77008,"parent_record_offset":76604,"ancestor_record_offset":75840,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":77092,"parent_record_offset":76348,"ancestor_record_offset":75840,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":513,"record_offset":78708,"parent_record_offset":78684,"ancestor_record_offset":75840,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":76296,"parent_record_offset":76268,"ancestor_record_offset":75840,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":78584,"parent_record_offset":78544,"ancestor_record_offset":75840,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":78684,"parent_record_offset":78660,"ancestor_record_offset":75840,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":77152,"parent_record_offset":77116,"ancestor_record_offset":75840,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":77176,"parent_record_offset":77152,"ancestor_record_offset":75840,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":77208,"parent_record_offset":77116,"ancestor_record_offset":75840,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":513,"record_offset":78812,"parent_record_offset":78788,"ancestor_record_offset":75840,"inlinee":26165,"source_line":168,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":513,"record_offset":78840,"parent_record_offset":78812,"ancestor_record_offset":75840,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":513,"record_offset":78404,"parent_record_offset":78368,"ancestor_record_offset":75840,"inlinee":26480,"source_line":170,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":70392,"rva":9791184,"va":5378500304}
// 编译实例 RVAs: 0x9566d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3419010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378500304,"raw_hex":"e87ed4cbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378500493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409231f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-211508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378500304,"raw_hex":"e8ccc5fcff","size":5,"soft_fail":false,"target_va":5378290160,"va":5378501663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4338701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378500304,"raw_hex":"e80d344200","size":5,"soft_fail":false,"target_va":5382840208,"va":5378501502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33039247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378500304,"raw_hex":"e88f23f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378501711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378500304,"raw_hex":"e80789ffff","size":5,"soft_fail":false,"target_va":5378471376,"va":5378501828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":75692,"parent_record_offset":70392,"ancestor_record_offset":70392,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":513,"record_offset":75776,"parent_record_offset":70392,"ancestor_record_offset":70392,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":513,"record_offset":75664,"parent_record_offset":70392,"ancestor_record_offset":70392,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":513,"record_offset":75804,"parent_record_offset":70392,"ancestor_record_offset":70392,"inlinee":26157,"source_line":92,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":513,"record_offset":70584,"parent_record_offset":70392,"ancestor_record_offset":70392,"inlinee":26478,"source_line":165,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":72256,"parent_record_offset":72232,"ancestor_record_offset":70392,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":72280,"parent_record_offset":72256,"ancestor_record_offset":70392,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":72232,"parent_record_offset":72140,"ancestor_record_offset":70392,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":75740,"parent_record_offset":75716,"ancestor_record_offset":70392,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70728,"parent_record_offset":70704,"ancestor_record_offset":70392,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":71248,"parent_record_offset":71212,"ancestor_record_offset":70392,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70776,"parent_record_offset":70752,"ancestor_record_offset":70392,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70752,"parent_record_offset":70728,"ancestor_record_offset":70392,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":71324,"parent_record_offset":71284,"ancestor_record_offset":70392,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":71284,"parent_record_offset":71248,"ancestor_record_offset":70392,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":71356,"parent_record_offset":71324,"ancestor_record_offset":70392,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":70704,"parent_record_offset":70680,"ancestor_record_offset":70392,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":70680,"parent_record_offset":70656,"ancestor_record_offset":70392,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":71604,"parent_record_offset":71580,"ancestor_record_offset":70392,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":71176,"parent_record_offset":71128,"ancestor_record_offset":70392,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":71212,"parent_record_offset":71176,"ancestor_record_offset":70392,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":71040,"parent_record_offset":70880,"ancestor_record_offset":70392,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":70632,"parent_record_offset":70608,"ancestor_record_offset":70392,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":71772,"parent_record_offset":71748,"ancestor_record_offset":70392,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":70656,"parent_record_offset":70632,"ancestor_record_offset":70392,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":513,"record_offset":72844,"parent_record_offset":72776,"ancestor_record_offset":70392,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":513,"record_offset":71580,"parent_record_offset":71128,"ancestor_record_offset":70392,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":513,"record_offset":71084,"parent_record_offset":71040,"ancestor_record_offset":70392,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":513,"record_offset":71128,"parent_record_offset":71084,"ancestor_record_offset":70392,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":71388,"parent_record_offset":71356,"ancestor_record_offset":70392,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":71420,"parent_record_offset":71388,"ancestor_record_offset":70392,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":513,"record_offset":71000,"parent_record_offset":70976,"ancestor_record_offset":70392,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":513,"record_offset":70976,"parent_record_offset":70952,"ancestor_record_offset":70392,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":71512,"parent_record_offset":71452,"ancestor_record_offset":70392,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":513,"record_offset":71652,"parent_record_offset":71628,"ancestor_record_offset":70392,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":513,"record_offset":71724,"parent_record_offset":71700,"ancestor_record_offset":70392,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":70928,"parent_record_offset":70880,"ancestor_record_offset":70392,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":71676,"parent_record_offset":71652,"ancestor_record_offset":70392,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":71700,"parent_record_offset":71676,"ancestor_record_offset":70392,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":72884,"parent_record_offset":72496,"ancestor_record_offset":70392,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":513,"record_offset":70952,"parent_record_offset":70928,"ancestor_record_offset":70392,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":513,"record_offset":72568,"parent_record_offset":72536,"ancestor_record_offset":70392,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":513,"record_offset":72536,"parent_record_offset":72496,"ancestor_record_offset":70392,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":71912,"parent_record_offset":71880,"ancestor_record_offset":70392,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":71628,"parent_record_offset":71604,"ancestor_record_offset":70392,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":71748,"parent_record_offset":71724,"ancestor_record_offset":70392,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":513,"record_offset":70608,"parent_record_offset":70584,"ancestor_record_offset":70392,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":71452,"parent_record_offset":71420,"ancestor_record_offset":70392,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":71484,"parent_record_offset":71452,"ancestor_record_offset":70392,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":72704,"parent_record_offset":72672,"ancestor_record_offset":70392,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":513,"record_offset":72672,"parent_record_offset":72640,"ancestor_record_offset":70392,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":513,"record_offset":75716,"parent_record_offset":75692,"ancestor_record_offset":70392,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":513,"record_offset":72932,"parent_record_offset":72908,"ancestor_record_offset":70392,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":71796,"parent_record_offset":71772,"ancestor_record_offset":70392,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":72908,"parent_record_offset":72884,"ancestor_record_offset":70392,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":513,"record_offset":72640,"parent_record_offset":72608,"ancestor_record_offset":70392,"inlinee":26480,"source_line":170,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7332
// 首条位置: {"module":584,"inlinee":29490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":81716,"rva":9794448,"va":5378503568}
// 编译实例 RVAs: 0x957390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3422152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e838c8cbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378503635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3422800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e8b0c5cbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378504299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-772373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e8eb36f4ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5378504368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33343389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e89dc7fc01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378503774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33036773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e8e519f801","size":5,"soft_fail":false,"target_va":5411540992,"va":5378504214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32842861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e86d24f501","size":5,"soft_fail":false,"target_va":5411347040,"va":5378504174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33036761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e8d919f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378504197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-759797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378503568,"raw_hex":"e80b68f4ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5378504000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":81924,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":83548,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":82072,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":83244,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":83632,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":83216,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":27004,"source_line":206,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":82664,"parent_record_offset":81716,"ancestor_record_offset":81716,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":81980,"parent_record_offset":81956,"ancestor_record_offset":81716,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":82004,"parent_record_offset":81980,"ancestor_record_offset":81716,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":83704,"parent_record_offset":83680,"ancestor_record_offset":81716,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":81956,"parent_record_offset":81924,"ancestor_record_offset":81716,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":83680,"parent_record_offset":83656,"ancestor_record_offset":81716,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":513,"record_offset":83656,"parent_record_offset":83632,"ancestor_record_offset":81716,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":83396,"parent_record_offset":83368,"ancestor_record_offset":81716,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":82256,"parent_record_offset":82228,"ancestor_record_offset":81716,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":83480,"parent_record_offset":83456,"ancestor_record_offset":81716,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":83456,"parent_record_offset":83424,"ancestor_record_offset":81716,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":83424,"parent_record_offset":83396,"ancestor_record_offset":81716,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":82316,"parent_record_offset":82284,"ancestor_record_offset":81716,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":82284,"parent_record_offset":82256,"ancestor_record_offset":81716,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":82352,"parent_record_offset":82316,"ancestor_record_offset":81716,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":82192,"parent_record_offset":82164,"ancestor_record_offset":81716,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":83572,"parent_record_offset":83548,"ancestor_record_offset":81716,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":82164,"parent_record_offset":82100,"ancestor_record_offset":81716,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":82792,"parent_record_offset":82764,"ancestor_record_offset":81716,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":82820,"parent_record_offset":82792,"ancestor_record_offset":81716,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":82764,"parent_record_offset":82736,"ancestor_record_offset":81716,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":82736,"parent_record_offset":82708,"ancestor_record_offset":81716,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":82708,"parent_record_offset":82664,"ancestor_record_offset":81716,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":82136,"parent_record_offset":82100,"ancestor_record_offset":81716,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":83596,"parent_record_offset":83572,"ancestor_record_offset":81716,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":83368,"parent_record_offset":83332,"ancestor_record_offset":81716,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":82228,"parent_record_offset":82192,"ancestor_record_offset":81716,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":82100,"parent_record_offset":82072,"ancestor_record_offset":81716,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":83300,"parent_record_offset":83272,"ancestor_record_offset":81716,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":83332,"parent_record_offset":83300,"ancestor_record_offset":81716,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":83272,"parent_record_offset":83244,"ancestor_record_offset":81716,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":82376,"parent_record_offset":82352,"ancestor_record_offset":81716,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":82400,"parent_record_offset":82376,"ancestor_record_offset":81716,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":82616,"parent_record_offset":82592,"ancestor_record_offset":81716,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":82424,"parent_record_offset":82400,"ancestor_record_offset":81716,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":82448,"parent_record_offset":82424,"ancestor_record_offset":81716,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":82592,"parent_record_offset":82192,"ancestor_record_offset":81716,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":82028,"parent_record_offset":82004,"ancestor_record_offset":81716,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":83748,"rva":9795280,"va":5378504400}
// 编译实例 RVAs: 0x9576d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3422980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e8fcc4cbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378504463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3423580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e8a4c2cbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378505079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-773025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e85f34f4ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5378505020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33036038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e80617f801","size":5,"soft_fail":false,"target_va":5411540992,"va":5378504949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32842106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e87a21f501","size":5,"soft_fail":false,"target_va":5411347040,"va":5378504929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33035987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e8d316f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378504971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9987831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e8f7669800","size":5,"soft_fail":false,"target_va":5388492688,"va":5378504852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-760489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378504400,"raw_hex":"e85765f4ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5378504692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":83956,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":85016,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":84684,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":85100,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":84656,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":27004,"source_line":206,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":84104,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/signifier_animation.rs
//     evidence={"module":513,"record_offset":84988,"parent_record_offset":83748,"ancestor_record_offset":83748,"inlinee":26557,"source_line":27,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":84012,"parent_record_offset":83988,"ancestor_record_offset":83748,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":84036,"parent_record_offset":84012,"ancestor_record_offset":83748,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":85172,"parent_record_offset":85148,"ancestor_record_offset":83748,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":83988,"parent_record_offset":83956,"ancestor_record_offset":83748,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":85148,"parent_record_offset":85124,"ancestor_record_offset":83748,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":513,"record_offset":85124,"parent_record_offset":85100,"ancestor_record_offset":83748,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84836,"parent_record_offset":84808,"ancestor_record_offset":83748,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84380,"parent_record_offset":84352,"ancestor_record_offset":83748,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84920,"parent_record_offset":84896,"ancestor_record_offset":83748,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84896,"parent_record_offset":84864,"ancestor_record_offset":83748,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84864,"parent_record_offset":84836,"ancestor_record_offset":83748,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84440,"parent_record_offset":84408,"ancestor_record_offset":83748,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":84408,"parent_record_offset":84380,"ancestor_record_offset":83748,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":84316,"parent_record_offset":84288,"ancestor_record_offset":83748,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":85040,"parent_record_offset":85016,"ancestor_record_offset":83748,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":84288,"parent_record_offset":84260,"ancestor_record_offset":83748,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":84232,"parent_record_offset":84204,"ancestor_record_offset":83748,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":84260,"parent_record_offset":84232,"ancestor_record_offset":83748,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":84204,"parent_record_offset":84176,"ancestor_record_offset":83748,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":84176,"parent_record_offset":84148,"ancestor_record_offset":83748,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":84148,"parent_record_offset":84104,"ancestor_record_offset":83748,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":85064,"parent_record_offset":85040,"ancestor_record_offset":83748,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":84808,"parent_record_offset":84772,"ancestor_record_offset":83748,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":84352,"parent_record_offset":84316,"ancestor_record_offset":83748,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":84740,"parent_record_offset":84712,"ancestor_record_offset":83748,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":84772,"parent_record_offset":84740,"ancestor_record_offset":83748,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":513,"record_offset":84712,"parent_record_offset":84684,"ancestor_record_offset":83748,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":84592,"parent_record_offset":84568,"ancestor_record_offset":83748,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":84568,"parent_record_offset":84316,"ancestor_record_offset":83748,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":84060,"parent_record_offset":84036,"ancestor_record_offset":83748,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x676E
// 首条位置: {"module":479,"inlinee":26478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":51040,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":50816,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":50784,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":51576,"parent_record_offset":51552,"ancestor_record_offset":50784,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":51528,"parent_record_offset":51312,"ancestor_record_offset":50784,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70728,"parent_record_offset":70704,"ancestor_record_offset":70584,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":51244,"parent_record_offset":51212,"ancestor_record_offset":50784,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70776,"parent_record_offset":70752,"ancestor_record_offset":70584,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":70752,"parent_record_offset":70728,"ancestor_record_offset":70584,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":51312,"parent_record_offset":51276,"ancestor_record_offset":50784,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":51276,"parent_record_offset":51244,"ancestor_record_offset":50784,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":51356,"parent_record_offset":51312,"ancestor_record_offset":50784,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":70704,"parent_record_offset":70680,"ancestor_record_offset":70584,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":50888,"parent_record_offset":50864,"ancestor_record_offset":50784,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":51660,"parent_record_offset":51632,"ancestor_record_offset":50784,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":51180,"parent_record_offset":51136,"ancestor_record_offset":50784,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":51212,"parent_record_offset":51180,"ancestor_record_offset":50784,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":51072,"parent_record_offset":51040,"ancestor_record_offset":50784,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":50840,"parent_record_offset":50816,"ancestor_record_offset":50784,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":52060,"parent_record_offset":52028,"ancestor_record_offset":50784,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":479,"record_offset":50864,"parent_record_offset":50840,"ancestor_record_offset":50784,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":479,"record_offset":52136,"parent_record_offset":52060,"ancestor_record_offset":50784,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":479,"record_offset":51632,"parent_record_offset":51136,"ancestor_record_offset":50784,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":479,"record_offset":51104,"parent_record_offset":51072,"ancestor_record_offset":50784,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":479,"record_offset":51136,"parent_record_offset":51104,"ancestor_record_offset":50784,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":479,"record_offset":51380,"parent_record_offset":51356,"ancestor_record_offset":50784,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":479,"record_offset":51404,"parent_record_offset":51380,"ancestor_record_offset":50784,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":50996,"parent_record_offset":50972,"ancestor_record_offset":50784,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":50972,"parent_record_offset":50948,"ancestor_record_offset":50784,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":479,"record_offset":51484,"parent_record_offset":51428,"ancestor_record_offset":50784,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":479,"record_offset":51716,"parent_record_offset":51688,"ancestor_record_offset":50784,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":479,"record_offset":51884,"parent_record_offset":51772,"ancestor_record_offset":50784,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":479,"record_offset":50924,"parent_record_offset":50816,"ancestor_record_offset":50784,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":479,"record_offset":51744,"parent_record_offset":51716,"ancestor_record_offset":50784,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":479,"record_offset":51772,"parent_record_offset":51744,"ancestor_record_offset":50784,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":479,"record_offset":52176,"parent_record_offset":51772,"ancestor_record_offset":50784,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":479,"record_offset":50948,"parent_record_offset":50924,"ancestor_record_offset":50784,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":51844,"parent_record_offset":51812,"ancestor_record_offset":50784,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":51812,"parent_record_offset":51772,"ancestor_record_offset":50784,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":51688,"parent_record_offset":51660,"ancestor_record_offset":50784,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":479,"record_offset":52028,"parent_record_offset":51884,"ancestor_record_offset":50784,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":51428,"parent_record_offset":51404,"ancestor_record_offset":50784,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":479,"record_offset":51456,"parent_record_offset":51428,"ancestor_record_offset":50784,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":51984,"parent_record_offset":51952,"ancestor_record_offset":50784,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":479,"record_offset":51952,"parent_record_offset":51920,"ancestor_record_offset":50784,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":479,"record_offset":52224,"parent_record_offset":52200,"ancestor_record_offset":50784,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":52100,"parent_record_offset":52060,"ancestor_record_offset":50784,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":479,"record_offset":52200,"parent_record_offset":52176,"ancestor_record_offset":50784,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":479,"record_offset":51920,"parent_record_offset":51884,"ancestor_record_offset":50784,"inlinee":26480,"source_line":170,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6770
// 首条位置: {"module":479,"inlinee":26480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":479,"record_offset":51952,"parent_record_offset":51920,"ancestor_record_offset":51920,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":479,"record_offset":51984,"parent_record_offset":51952,"ancestor_record_offset":51920,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00479.jsonl.gz"}
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
// 完整结构及原始类型记录: adjust_wall_ui_mode.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: adjust_wall_ui_mode.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: adjust_wall_ui_mode.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 信息 ID: 4bbeef8cfd9d7f380901e06e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height; module=584, inline_record=77664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 信息 ID: ba43278c873e9365078eed12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0xb3045a -> 0x956cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 信息 ID: 8026cd7db13d276666a107d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0xb3041b -> 0x9566d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 信息 ID: 1d9fa432d012e258726c4943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation; module=479, inline_record=50784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=70584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=70836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=71880; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=73012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=74484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=513, inline_record=77248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 信息 ID: 1b13738a13bb25c6f7bd2f87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation; module=479, inline_record=51920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=72640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=74100; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=513, inline_record=75292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=513, inline_record=78404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 信息 ID: ca9409f3f2b9455eb64b8771
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x94f433 -> 0x957390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 信息 ID: 35d507bf932978d4f9c0f9f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x94f4dd -> 0x9576d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
